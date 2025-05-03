use crate::{CommandArgs, Compiler, Language, Runner};

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
    },
};
