use std::{
    env,
    ffi::{CString, c_int},
    fs, iter,
    path::Path,
    time::{Duration, Instant},
};

use libseccomp::{ScmpAction, ScmpFilterContext, ScmpSyscall};
use nix::{
    libc::{self, WEXITSTATUS, WSTOPPED, WTERMSIG, wait4},
    sys::{
        resource::{Resource, setrlimit},
        signal::{SaFlags, SigAction, SigHandler, SigSet, Signal, sigaction},
    },
    unistd::{ForkResult, alarm, dup2_stderr, dup2_stdin, dup2_stdout, execvp, fork},
};
use state_shift::{impl_state, type_state};

use crate::{
    CommandArgs, Error, Result,
    metrics::{Metrics, Rusage, get_default_rusage},
};

extern "C" fn signal_handler(_: nix::libc::c_int) {}
#[derive(Debug, Clone, Copy)]
pub struct RlimitConfig {
    pub resource: Resource,
    pub soft_limit: u64,
    pub hard_limit: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct SandboxConfig<'a> {
    pub scmp_black_list: &'a [&'a str],
    pub rlimit_configs: &'a [RlimitConfig],
}

impl<'a> SandboxConfig<'a> {
    fn apply(&self) -> Result<()> {
        for rlimit in self.rlimit_configs {
            setrlimit(rlimit.resource, rlimit.soft_limit, rlimit.hard_limit)?;
        }

        let mut scmp_filter = ScmpFilterContext::new(ScmpAction::Allow)?;
        for s in self.scmp_black_list {
            let syscall = ScmpSyscall::from_name(s)?;
            scmp_filter.add_rule_exact(ScmpAction::KillProcess, syscall)?;
        }

        scmp_filter.load()?;

        Ok(())
    }
}

#[type_state(
    states = (Initial, Running),
    slots = (Initial)
)]
#[derive(Debug)]
pub struct Sandbox<'a> {
    config: SandboxConfig<'a>,
    project_path: &'a Path,
    args: CommandArgs<'a>,
    stdin: &'a Path,
    stdout: &'a Path,
    stderr: &'a Path,
    time_limit: Duration,

    child_pid: i32,
    start: Instant,
}

#[impl_state]
impl<'a> Sandbox<'a> {
    #[require(Initial)]
    pub fn new(
        config: SandboxConfig<'a>,
        project_path: &'a Path,
        args: CommandArgs<'a>,
        stdin: &'a Path,
        stdout: &'a Path,
        stderr: &'a Path,
        time_limit: Duration,
    ) -> Self {
        Self {
            config,
            project_path,
            args,
            stdin,
            stdout,
            stderr,
            time_limit,
            child_pid: -1,
            start: Instant::now(),
            _state: (::core::marker::PhantomData),
        }
    }

    #[require(Initial)]
    fn load_io(&self) -> Result<()> {
        let stdin = fs::OpenOptions::new().read(true).open(self.stdin)?;
        dup2_stdin(stdin)?;

        let stdout = fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(self.stdout)?;
        dup2_stdout(stdout)?;

        let stderr = fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(self.stderr)?;
        dup2_stderr(stderr)?;

        Ok(())
    }

    #[require(Initial)]
    #[switch_to(Running)]
    pub fn spawn(self) -> Result<Sandbox<'a, Running>> {
        unsafe {
            sigaction(
                Signal::SIGALRM,
                &SigAction::new(
                    SigHandler::Handler(signal_handler),
                    SaFlags::empty(),
                    SigSet::empty(),
                ),
            )
            .unwrap();
        }

        let start = Instant::now();
        match unsafe { fork() } {
            Ok(ForkResult::Parent { child, .. }) => Ok(Sandbox {
                config: self.config,
                project_path: self.project_path,
                args: self.args,
                stdin: self.stdin,
                stdout: self.stdout,
                stderr: self.stderr,
                time_limit: self.time_limit,
                child_pid: child.as_raw(),
                start,
                _state: (::core::marker::PhantomData),
            }),
            // child process should not return to do things outside `spawn()`
            Ok(ForkResult::Child) => {
                if env::set_current_dir(self.project_path).is_err() {
                    eprintln!("Failed to load change to project directory");
                    unsafe { libc::_exit(100) };
                }

                if self.load_io().is_err() {
                    eprintln!("Failed to load I/O");
                    unsafe { libc::_exit(1) };
                }

                if self.config.apply().is_err() {
                    eprintln!("Failed to load config");
                    unsafe { libc::_exit(1) };
                }

                alarm::set(self.time_limit.as_secs() as u32);

                let CommandArgs { binary, args } = self.args;
                let args: Vec<_> = iter::once(binary)
                    .chain(args.iter().copied())
                    .map(|arg| CString::new(arg.as_bytes()).unwrap())
                    .collect();
                let binary = CString::new(binary.as_bytes()).unwrap();

                let error = execvp(&binary, &args).unwrap_err();
                eprintln!("{}", error);

                unsafe { libc::_exit(0) };
            }
            Err(e) => Err(e.into()),
        }
    }

    #[require(Running)]
    pub fn wait(self) -> Result<Metrics> {
        let mut status: c_int = 0;
        let mut usage = get_default_rusage();
        unsafe {
            wait4(self.child_pid, &mut status, WSTOPPED, &mut usage);
        }

        let error = fs::read_to_string(self.stderr)?;
        if !error.is_empty() {
            return Err(Error::Runtime { message: error });
        }

        let output = fs::read_to_string(self.stdout)?.trim().to_string();
        Ok(Metrics {
            exit_status: status,
            exit_signal: WTERMSIG(status),
            exit_code: WEXITSTATUS(status),
            real_time_cost: self.start.elapsed(),
            resource_usage: Rusage::from(usage),
            output,
        })
    }
}
