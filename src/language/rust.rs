use crate::{CommandArgs, Compiler, Language, Runner};

use super::DEFAULT_SANDBOX_CONFIG;

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
        sandbox_config: DEFAULT_SANDBOX_CONFIG,
    },
};
