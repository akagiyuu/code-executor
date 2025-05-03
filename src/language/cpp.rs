use crate::{CommandArgs, Compiler, Language, Runner};

use super::{DEFAULT_MAX_CPU_PERCENTAGE, DEFAULT_MAX_MEMORY};

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
        max_memory: DEFAULT_MAX_MEMORY,
        max_cpu_percentage: DEFAULT_MAX_CPU_PERCENTAGE,
    },
};
