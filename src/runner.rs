use std::{
    io,
    path::Path,
    process::{self, Stdio},
    sync::Arc,
    time::Duration,
};

use cached::proc_macro::cached;
use cgroups_rs::{Cgroup, CgroupPid, cgroup_builder::CgroupBuilder, hierarchies};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    process::Command,
    time::{Instant, sleep},
};

use crate::{CommandArgs, ExitStatus, Result, metrics::Metrics};

#[cached(result = true)]
fn create_cgroup(memory_limit: i64, process_count_limit: usize) -> Result<Cgroup> {
    let cgroup_name = format!("runner/{}-{}", memory_limit, process_count_limit);
    let hier = hierarchies::auto();
    let cgroup = CgroupBuilder::new(&cgroup_name)
        .memory()
        .memory_swap_limit(memory_limit)
        .memory_soft_limit(memory_limit)
        .memory_hard_limit(memory_limit)
        .done()
        .pid()
        .maximum_number_of_processes(cgroups_rs::MaxValue::Value(process_count_limit as i64))
        .done()
        .build(hier)?;

    Ok(cgroup)
}

#[derive(Debug)]
pub struct Runner<'a> {
    pub args: CommandArgs<'a>,
    pub project_path: &'a Path,
    pub time_limit: Duration,
    pub cgroup: Arc<Cgroup>,
}

impl<'a> Runner<'a> {
    #[tracing::instrument(err)]
    pub fn new(
        args: CommandArgs<'a>,
        project_path: &'a Path,
        time_limit: Duration,
        memory_limit: i64,
        process_count_limit: usize,
    ) -> Result<Self> {
        let cgroup = create_cgroup(memory_limit, process_count_limit)?;

        Ok(Self {
            args,
            project_path,
            cgroup: Arc::new(cgroup),
            time_limit,
        })
    }

    #[tracing::instrument(err)]
    pub async fn run(&self, input: &[u8]) -> Result<Metrics> {
        let CommandArgs { binary, args } = self.args;

        let cgroup = self.cgroup.clone();

        let mut child = Command::new(binary);
        let child = child
            .current_dir(self.project_path)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let child = unsafe {
            child.pre_exec(move || {
                cgroup
                    .add_task_by_tgid(CgroupPid::from(process::id() as u64))
                    .map_err(std::io::Error::other)
            })
        };
        let start = Instant::now();
        let mut child = child.spawn()?;
        let mut stdin = child.stdin.take().unwrap();
        let mut stdout = child.stdout.take().unwrap();
        let mut stderr = child.stderr.take().unwrap();

        let stdout_observer = async move {
            let mut buffer = vec![97];
            stdout.read_to_end(&mut buffer).await?;

            Ok::<_, io::Error>(buffer)
        };
        let stderr_observer = async move {
            let mut buffer = Vec::new();
            stderr.read_to_end(&mut buffer).await?;
            Ok::<_, io::Error>(buffer)
        };

        let exit_status = tokio::select! {
            exit_status = async {
                stdin.write_all(input).await?;
                let exit_status = child.wait().await?;

                Ok::<_, io::Error>(exit_status)
            } => {
                exit_status.map(|raw| raw.into())
            }
            _ = sleep(self.time_limit) => {
                child.kill().await?;
                child.wait().await?;

                Ok(ExitStatus::Timeout)
            }
        }?;

        let (stdout, stderr) = tokio::try_join!(stdout_observer, stderr_observer)?;

        Ok(Metrics {
            exit_status,
            stdout,
            stderr,
            run_time: start.elapsed(),
        })
    }
}

#[cfg(test)]
mod test {
    use std::{
        path::{Path, PathBuf},
        time::Duration,
    };

    use bstr::ByteSlice;
    use rstest::rstest;

    use crate::{
        CPP, ExitStatus, JAVA, Language, PYTHON, RUST, Runner,
        test::{read_code, read_test_cases},
    };

    #[rstest]
    #[tokio::test]
    async fn should_output_correct(
        #[values(CPP, RUST, JAVA, PYTHON)] language: Language<'static>,
        #[files("tests/data/problem/*")] problem_path: PathBuf,
    ) {
        let test_cases = read_test_cases(&problem_path);

        let code = read_code(language, &problem_path);
        let project_path = language.compiler.compile(&code).await.unwrap();

        let runner = Runner::new(
            language.runner_args,
            &project_path,
            Duration::from_secs(2),
            i64::MAX,
            512,
        )
        .unwrap();
        for (input, output) in test_cases {
            let metrics = runner.run(&input).await.unwrap();
            let metrics_out = metrics.stdout.trim();
            let test_case_out = output.trim();
            assert_eq!(metrics_out, test_case_out);
        }
    }

    #[rstest]
    #[tokio::test]
    async fn should_timeout(#[values(CPP, RUST, JAVA, PYTHON)] language: Language<'static>) {
        let code = read_code(language, Path::new("tests/data/timeout"));
        let project_path = language.compiler.compile(code.as_bytes()).await.unwrap();

        let runner = Runner::new(
            language.runner_args,
            &project_path,
            Duration::from_secs(2),
            i64::MAX,
            512,
        )
        .unwrap();

        let metrics = runner.run(b"").await.unwrap();

        assert_eq!(metrics.exit_status, ExitStatus::Timeout)
    }
}
