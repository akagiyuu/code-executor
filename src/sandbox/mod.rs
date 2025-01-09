mod config;

use std::ffi::{CString, c_int};
use std::fs::{self, File};
use std::io;
use std::marker::PhantomData;
use std::os::fd::AsRawFd as _;
use std::path::{Path, PathBuf};
use std::time::Instant;
use std::{env, iter};

use bon::Builder;
use nix::libc::{self, WEXITSTATUS, WSTOPPED, WTERMSIG, rusage, wait4};
use nix::sys::signal::{SaFlags, SigAction, SigHandler, SigSet, Signal, sigaction};
use nix::unistd::{ForkResult, alarm, dup2, execvp, fork};

pub use config::*;

use crate::{CommandArgs, Error, Result, runner};

extern "C" fn signal_handler(_: nix::libc::c_int) {}

pub struct Init;
pub struct Running;

#[derive(Debug, Builder)]
pub struct Sandbox<'a, State = Init> {
    config: Config<'a>,

    #[builder(into)]
    project_path: PathBuf,

    #[builder(with = |input_path: impl AsRef<Path>| -> io::Result<_> {
        fs::OpenOptions::new().read(true).open(input_path)
    })]
    input: File,

    output_path: &'a Path,

    #[builder(default = fs::OpenOptions::new().create(true).truncate(true).write(true).open(output_path).unwrap())]
    output: File,

    error_path: &'a Path,

    #[builder(default = fs::OpenOptions::new().create(true).truncate(true).write(true).open(error_path).unwrap())]
    error: File,

    #[builder(default = -1)]
    child_pid: i32,

    #[builder(default = Instant::now())]
    begin_time: Instant,

    #[builder(default)]
    state: PhantomData<State>,
}

impl<'a> Sandbox<'a, Init> {
    fn load_io(&self) -> Result<()> {
        let stdin_raw_fd = io::stdin().as_raw_fd();
        dup2(self.input.as_raw_fd(), stdin_raw_fd)?;

        let stdout_raw_fd = io::stdout().as_raw_fd();
        dup2(self.output.as_raw_fd(), stdout_raw_fd)?;

        let sterr_raw_fd = io::stderr().as_raw_fd();
        dup2(self.error.as_raw_fd(), sterr_raw_fd)?;

        Ok(())
    }

    /// WARNING:   
    /// Unsafe to use `println!()` (or `unwrap()`) in child process.
    /// See more in `fork()` document.
    pub fn spawn(self) -> Result<Sandbox<'a, Running>> {
        let now = Instant::now();
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
        match unsafe { fork() } {
            Ok(ForkResult::Parent { child, .. }) => Ok(Sandbox {
                config: self.config,
                project_path: self.project_path,
                input: self.input,
                output_path: self.output_path,
                output: self.output,
                error_path: self.error_path,
                error: self.error,
                child_pid: child.as_raw(),
                begin_time: now,
                state: PhantomData::<Running>,
            }),
            // child process should not return to do things outside `spawn()`
            Ok(ForkResult::Child) => {
                if env::set_current_dir(&self.project_path).is_err() {
                    eprintln!("Failed to load change to project directory");
                    unsafe { libc::_exit(100) };
                }

                if self.load_io().is_err() {
                    eprintln!("Failed to load I/O");
                    unsafe { libc::_exit(1) };
                }

                if self.config.load().is_err() {
                    eprintln!("Failed to load config");
                    unsafe { libc::_exit(1) };
                }

                alarm::set(self.config.time_limit.as_secs() as u32);

                let CommandArgs { binary, args } = self.config.args;
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
}

impl Sandbox<'_, Running> {
    pub fn wait(self) -> Result<runner::Metrics> {
        let mut status: c_int = 0;
        let mut usage: rusage = runner::get_default_rusage();
        unsafe {
            wait4(self.child_pid, &mut status, WSTOPPED, &mut usage);
        }

        let error = fs::read_to_string(self.error_path)?;
        if !error.is_empty() {
            return Err(Error::Runtime { message: error });
        }

        let output = fs::read_to_string(self.output_path)?.trim().to_string();
        Ok(runner::Metrics {
            exit_status: status,
            exit_signal: WTERMSIG(status),
            exit_code: WEXITSTATUS(status),
            real_time_cost: self.begin_time.elapsed(),
            resource_usage: runner::Rusage::from(usage),
            output,
        })
    }
}
