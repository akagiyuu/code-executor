use crate::{CommandArgs, Compiler, Language, Runner};

use super::{DEFAULT_MAX_CPU_PERCENTAGE, DEFAULT_MAX_MEMORY};

pub const RUST: Language = Language {
    compiler: Compiler {
        main_file: "main.rs",
        args: Some(CommandArgs {
            binary: "rustc",
            args: &["-O", "main.rs"],
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
