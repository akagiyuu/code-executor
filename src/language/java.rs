use crate::{CommandArgs, Compiler, Language};

pub const JAVA: Language = Language {
    compiler: Compiler {
        main_file: "Main.java",
        args: Some(CommandArgs {
            binary: "javac",
            args: &["Main.java"],
        }),
    },
    runner_args: CommandArgs {
        binary: "java",
        args: &["Main"],
    },
};
