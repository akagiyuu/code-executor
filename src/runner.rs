use std::{
    path::Path,
    process::{self, Stdio},
    sync::Arc,
    time::Duration,
};

use cached::proc_macro::cached;
use cgroups_rs::{Cgroup, CgroupPid, cgroup_builder::CgroupBuilder, hierarchies};
use tokio::{
    io::AsyncWriteExt,
    process::Command,
    time::{Instant, timeout},
};

use crate::{CommandArgs, Result, metrics::Metrics};

#[cached(result = true)]
fn create_cgroup(memory_limit: i64) -> Result<Cgroup> {
    let cgroup_name = format!("runner/{}", memory_limit);
    let hier = hierarchies::auto();
    let cgroup = CgroupBuilder::new(&cgroup_name)
        .memory()
        .memory_swap_limit(memory_limit)
        .memory_soft_limit(memory_limit)
        .memory_hard_limit(memory_limit)
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
    ) -> Result<Self> {
        let cgroup = create_cgroup(memory_limit)?;

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

        let child_stdin = child.stdin.as_mut().unwrap();
        child_stdin.write_all(input).await?;

        let output = timeout(self.time_limit, child.wait_with_output()).await??;

        Ok(Metrics {
            run_time: start.elapsed(),
            exit_status: output.status,
            stdout: output.stdout,
            stderr: output.stderr,
        })
    }
}
