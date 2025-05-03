mod compiler;
mod error;
mod language;
mod metrics;
mod runner;
mod sandbox;
mod util;

pub use compiler::Compiler;
pub use error::*;
pub use language::*;
pub use runner::Runner;
pub use sandbox::{SandboxConfig, RlimitConfig};

#[derive(Debug, Clone, Copy)]
pub struct CommandArgs<'a> {
    pub binary: &'a str,
    pub args: &'a [&'a str],
}

#[derive(Debug, Clone, Copy)]
pub struct Language<'a> {
    pub compiler: Compiler<'a>,
    pub runner: Runner<'a>,
}
