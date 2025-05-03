use crate::{CommandArgs, Compiler, Language};

pub const CPP: Language = Language {
    compiler: Compiler {
        main_file: "main.cpp",
        args: Some(CommandArgs {
            binary: "g++",
            args: &["-o", "main", "main.cpp"],
        }),
    },
    runner_args: CommandArgs {
        binary: "./main",
        args: &[],
    },
};
