use std::{
    hash::{DefaultHasher, Hash, Hasher},
    path::Path,
    process::{self, Stdio},
    time::Duration,
};

use cgroups_rs::{Cgroup, CgroupPid, cgroup_builder::CgroupBuilder, hierarchies};
use tokio::{
    io::AsyncWriteExt,
    process::Command,
    time::{Instant, timeout},
};

use crate::{CommandArgs, Result, metrics::Metrics};

#[derive(Debug, Clone, Copy, Hash)]
pub struct Runner<'a> {
    pub args: CommandArgs<'a>,
}

impl Runner<'_> {
    fn get_cgroup_name(&self, project_path: &Path) -> String {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        project_path.hash(&mut hasher);

        format!("runner/{}", hasher.finish())
    }

    #[tracing::instrument(err)]
    fn create_cgroup(&self, project_path: &Path, max_memory: i64) -> Result<Cgroup> {
        let hier = hierarchies::auto();
        let cgroup = CgroupBuilder::new(&self.get_cgroup_name(project_path))
            .memory()
            .memory_swap_limit(max_memory)
            .memory_soft_limit(max_memory)
            .memory_hard_limit(max_memory)
            .done()
            .build(hier)?;
        Ok(cgroup)
    }

    #[tracing::instrument(err)]
    pub async fn run(
        &self,
        project_path: &Path,
        input: &str,
        time_limit: Duration,
        max_memory: i64,
    ) -> Result<Metrics> {
        let CommandArgs { binary, args } = self.args;

        let cgroup = self.create_cgroup(project_path, max_memory)?;

        let mut child = Command::new(binary);
        let child = child
            .current_dir(project_path)
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
        child_stdin.write_all(input.as_bytes()).await?;

        let output = timeout(time_limit, child.wait_with_output()).await??;

        Ok(Metrics {
            run_time: start.elapsed(),
            exit_status: output.status,
            stdout: String::from_utf8(output.stdout)?.trim().to_string(),
            stderr: String::from_utf8(output.stderr)?,
        })
    }
}
