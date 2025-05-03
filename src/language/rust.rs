use crate::{CommandArgs, Compiler, Language};

pub const RUST: Language = Language {
    compiler: Compiler {
        main_file: "main.rs",
        args: Some(CommandArgs {
            binary: "rustc",
            args: &["-O", "main.rs"],
        }),
    },
    runner_args: CommandArgs {
        binary: "./main",
        args: &[],
    },
};
