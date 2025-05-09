mod compiler;
mod error;
mod language;
mod metrics;
mod runner;
mod util;

pub use compiler::Compiler;
pub use error::*;
pub use language::*;
pub use metrics::Metrics;
pub use runner::Runner;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct CommandArgs<'a> {
    pub binary: &'a str,
    pub args: &'a [&'a str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Language<'a> {
    pub compiler: Compiler<'a>,
    pub runner_args: CommandArgs<'a>,
}
