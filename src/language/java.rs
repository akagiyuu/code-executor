use crate::{CommandArgs, Compiler, Language, Runner};

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
    },
};
