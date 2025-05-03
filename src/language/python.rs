use crate::{CommandArgs, Compiler, Language, Runner};

use super::{DEFAULT_MAX_CPU_PERCENTAGE, DEFAULT_MAX_MEMORY};

pub const PYTHON: Language = Language {
    compiler: Compiler {
        main_file: "main.py",
        args: None,
    },
    runner: Runner {
        args: CommandArgs {
            binary: "python",
            args: &["main.py"],
        },
        max_memory: DEFAULT_MAX_MEMORY,
        max_cpu_percentage: DEFAULT_MAX_CPU_PERCENTAGE,
    },
};
