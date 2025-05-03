use crate::{CommandArgs, Compiler, Language, Runner};

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
    },
};
