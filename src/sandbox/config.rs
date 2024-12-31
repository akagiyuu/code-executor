use std::time::Duration;

use bon::Builder;
use libseccomp::{ScmpAction, ScmpFilterContext, ScmpSyscall};
use nix::sys::resource::{Resource, setrlimit};

use crate::{CommandArgs, Result};

#[derive(Debug, Clone, Builder)]
pub struct RlimitConfig {
    pub resource: Resource,
    pub soft_limit: u64,
    pub hard_limit: u64,
}

impl RlimitConfig {
    fn apply(&self) -> Result<()> {
        setrlimit(self.resource, self.soft_limit, self.hard_limit)?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Config<'a> {
    pub scmp_black_list: &'a [&'a str],
    pub rlimit_configs: &'a [RlimitConfig],
    pub time_limit: Duration,
    pub args: CommandArgs<'a>,
}

impl Config<'_> {
    pub fn load(&self) -> Result<()> {
        for rlimit_config in self.rlimit_configs {
            rlimit_config.apply()?;
        }

        let mut scmp_filter = ScmpFilterContext::new_filter(ScmpAction::Allow)?;
        for s in self.scmp_black_list {
            let syscall = ScmpSyscall::from_name(s)?;
            scmp_filter.add_rule_exact(ScmpAction::KillProcess, syscall)?;
        }

        scmp_filter.load()?;

        Ok(())
    }
}
