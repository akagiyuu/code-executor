mod compiler;
mod error;
mod metrics;
pub mod runner;
mod sandbox;
mod util;
mod language;

use bon::Builder;

pub use compiler::Compiler;
pub use error::*;
pub use runner::Runner;
pub use language::*;

#[derive(Debug, Clone, Copy, Builder)]
pub struct CommandArgs<'a> {
    pub binary: &'a str,
    pub args: &'a [&'a str],
}

pub struct Language<'a> {
    pub compiler: Compiler<'a>,
    pub runner: Runner<'a>,
}
