use std::{path::Path, time::Duration};

use bon::Builder;

use crate::{
    CommandArgs, Result,
    metrics::Metrics,
    sandbox::{Sandbox, SandboxConfig},
};

#[derive(Debug, Clone, Copy, Builder)]
pub struct Runner<'a> {
    pub args: CommandArgs<'a>,
    pub sandbox_config: SandboxConfig<'a>,
}

impl Runner<'_> {
    pub fn run(&self, project_path: &Path, stdin: &Path, time_limit: Duration) -> Result<Metrics> {
        let stdout = project_path.join("stdout.txt");
        let stderr = project_path.join("stderr.txt");

        let sandbox = Sandbox::new(
            self.sandbox_config,
            project_path,
            self.args,
            stdin,
            stdout.as_path(),
            stderr.as_path(),
            time_limit,
        );

        let sandbox = sandbox.spawn()?;
        sandbox.wait()
    }
}
