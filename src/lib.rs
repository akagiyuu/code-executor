mod compiler;
mod error;
mod language;
mod metrics;
mod runner;
mod util;

pub use compiler::*;
pub use error::*;
pub use language::*;
pub use metrics::*;
pub use runner::*;

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

#[cfg(test)]
mod test {
    use std::{fs, path::Path};

    use crate::Language;

    pub fn read_test_cases(problem_path: &Path) -> Vec<(Vec<u8>, Vec<u8>)> {
        let test_cases: Vec<_> = fs::read_dir(problem_path)
            .unwrap()
            .filter_map(|entry| {
                let entry = entry.unwrap();
                let path = entry.path();
                if !path.is_dir() {
                    return None;
                }

                let input_path = path.join("in.txt");
                let input = fs::read(input_path).unwrap();

                let output_path = path.join("out.txt");
                let output = fs::read(output_path).unwrap();

                Some((input, output))
            })
            .collect();

        test_cases
    }

    pub fn read_code(language: Language, problem_path: &Path) -> Vec<u8> {
        fs::read(problem_path.join(language.compiler.main_file)).unwrap()
    }
}
