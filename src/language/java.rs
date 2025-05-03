use crate::{CommandArgs, Compiler, Language, Runner};

use super::{DEFAULT_MAX_CPU_PERCENTAGE, DEFAULT_MAX_MEMORY};

pub const JAVA: Language = Language {
    compiler: Compiler {
        main_file: "Main.java",
        args: Some(CommandArgs {
            binary: "javac",
            args: &["Main.java"],
        }),
    },
    runner: Runner {
        args: CommandArgs {
            binary: "java",
            args: &["Main"],
        },
        max_memory: DEFAULT_MAX_MEMORY,
        max_cpu_percentage: DEFAULT_MAX_CPU_PERCENTAGE,
    },
};
