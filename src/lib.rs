mod error;
pub mod runner;
mod sandbox;
mod util;

use std::time::Duration;

use bon::Builder;

pub use error::*;
use nix::sys::resource::Resource;
use runner::Runner;
use sandbox::RlimitConfig;

#[derive(Debug, Clone, Copy, Builder)]
pub struct CommandArgs<'a> {
    pub binary: &'a str,
    pub args: &'a [&'a str],
}

const DEFAULT_RLIMIT_CONFIGS: &[RlimitConfig] = &[
    RlimitConfig {
        resource: Resource::RLIMIT_STACK,
        soft_limit: 1024 * 1024 * 1024 * 1024,
        hard_limit: 1024 * 1024 * 1024 * 1024,
    },
    RlimitConfig {
        resource: Resource::RLIMIT_AS,
        soft_limit: 1024 * 1024 * 1024 * 1024,
        hard_limit: 1024 * 1024 * 1024 * 1024,
    },
    RlimitConfig {
        resource: Resource::RLIMIT_CPU,
        soft_limit: 60,
        hard_limit: 90,
    },
    // RlimitConfig {
    //     resource: Resource::RLIMIT_NPROC,
    //     soft_limit: 1,
    //     hard_limit: 1,
    // },
    RlimitConfig {
        resource: Resource::RLIMIT_FSIZE,
        soft_limit: 1024,
        hard_limit: 1024,
    },
];

const DEFAULT_SCMP_BLACK_LIST: &[&str] = &["fork", "vfork"];

pub const CPP_RUNNER: Runner = Runner {
    main_file: "main.cpp",
    compiler_args: Some(CommandArgs {
        binary: "g++",
        args: &["-o", "main", "main.cpp"],
    }),
    sandbox_config: sandbox::Config {
        scmp_black_list: DEFAULT_SCMP_BLACK_LIST,
        rlimit_configs: DEFAULT_RLIMIT_CONFIGS,
        time_limit: Duration::from_secs(2),
        args: CommandArgs {
            binary: "./main",
            args: &[],
        },
    },
};

pub const PYTHON_RUNNER: Runner = Runner {
    main_file: "main.py",
    compiler_args: None,
    sandbox_config: sandbox::Config {
        scmp_black_list: DEFAULT_SCMP_BLACK_LIST,
        rlimit_configs: DEFAULT_RLIMIT_CONFIGS,
        time_limit: Duration::from_secs(10),
        args: CommandArgs {
            binary: "python",
            args: &["main.py"],
        },
    },
};

pub const JAVA_RUNNER: Runner = Runner {
    main_file: "Main.java",
    compiler_args: Some(CommandArgs {
        binary: "javac",
        args: &["Main.java"],
    }),
    sandbox_config: sandbox::Config {
        scmp_black_list: DEFAULT_SCMP_BLACK_LIST,
        rlimit_configs: DEFAULT_RLIMIT_CONFIGS,
        time_limit: Duration::from_secs(4),
        args: CommandArgs {
            binary: "java",
            args: &["Main"],
        },
    },
};

pub const RUST_RUNNER: Runner = Runner {
    main_file: "main.rs",
    compiler_args: Some(CommandArgs {
        binary: "rustc",
        args: &["-O", "main.rs"],
    }),
    sandbox_config: sandbox::Config {
        scmp_black_list: DEFAULT_SCMP_BLACK_LIST,
        rlimit_configs: DEFAULT_RLIMIT_CONFIGS,
        time_limit: Duration::from_secs(2),
        args: CommandArgs {
            binary: "./main",
            args: &[],
        },
    },
};

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::{fs, io::Write};

    use anyhow::{Error, Result};
    use rstest::rstest;
    use tempfile::NamedTempFile;

    use crate::RUST_RUNNER;
    use crate::{
        CPP_RUNNER, JAVA_RUNNER, PYTHON_RUNNER,
        runner::{self, Runner},
    };

    fn read_code(problem_path: &Path, runner: &Runner<'_>) -> Result<String> {
        fs::read_to_string(problem_path.join(runner.main_file)).map_err(Error::from)
    }

    #[rstest]
    fn test_hello_world(
        #[values(CPP_RUNNER, RUST_RUNNER, JAVA_RUNNER, PYTHON_RUNNER)] runner: Runner,
    ) -> Result<()> {
        const CODE_PATH: &str = "test/hello-world";
        const CORRECT_OUTPUT: &str = "Hello World";

        let code_path = Path::new(CODE_PATH);
        let input = NamedTempFile::new()?;

        let code = read_code(code_path, &runner)?;

        let metrics = runner.run(&code, input.path())?;
        assert_eq!(
            metrics.output,
            runner::Output::Success(CORRECT_OUTPUT.to_string())
        );

        Ok(())
    }

    const RANDOM_ITER: usize = 10;

    #[rstest]
    fn test_add(#[values(CPP_RUNNER, RUST_RUNNER, JAVA_RUNNER, PYTHON_RUNNER)] runner: Runner) -> Result<()> {
        const CODE_PATH: &str = "test/add";

        let code_path = Path::new(CODE_PATH);
        let code = read_code(code_path, &runner)?;

        for _ in 0..RANDOM_ITER {
            let [a, b]: [i16; 2] = rand::random();
            let mut input = NamedTempFile::new()?;
            input.write_fmt(format_args!("{}\n{}\n", a, b))?;

            let metrics = runner.run(&code, input.path()).unwrap();
            assert_eq!(
                metrics.output,
                runner::Output::Success((a as i32 + b as i32).to_string())
            );
        }

        Ok(())
    }
}
