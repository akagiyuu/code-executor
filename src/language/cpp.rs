use crate::{CommandArgs, Compiler, Language, Runner};

use super::DEFAULT_SANDBOX_CONFIG;

pub const CPP: Language = Language {
    compiler: Compiler {
        main_file: "main.cpp",
        args: Some(CommandArgs {
            binary: "g++",
            args: &["-o", "main", "main.cpp"],
        }),
    },
    runner: Runner {
        args: CommandArgs {
            binary: "./main",
            args: &[],
        },
        sandbox_config: DEFAULT_SANDBOX_CONFIG,
    },
};
