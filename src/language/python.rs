use crate::{CommandArgs, Compiler, Language};

pub const PYTHON: Language = Language {
    compiler: Compiler {
        main_file: "main.py",
        args: None,
    },
    runner_args: CommandArgs {
        binary: "python",
        args: &["main.py"],
    },
};
