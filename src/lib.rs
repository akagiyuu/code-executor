mod compiler;
mod error;
mod language;
mod metrics;
pub mod runner;
mod sandbox;
mod util;

pub use compiler::Compiler;
pub use error::*;
pub use language::*;
pub use runner::Runner;

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
