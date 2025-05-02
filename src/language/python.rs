use crate::{CommandArgs, Compiler, Language, Runner};

use super::DEFAULT_SANDBOX_CONFIG;

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
        sandbox_config: DEFAULT_SANDBOX_CONFIG,
    },
};
