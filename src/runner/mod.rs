mod metrics;

use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

pub use metrics::*;

use crate::{
    CommandArgs, Error, Result,
    sandbox::{self, Sandbox},
    util,
};

#[derive(Debug)]
pub struct Runner<'a> {
    pub main_file: &'a str,
    pub compiler_args: Option<CommandArgs<'a>>,
    pub sandbox_config: sandbox::Config<'a>,
}

impl Runner<'_> {
    pub fn init_project(&self, code: &str) -> Result<PathBuf> {
        let project_path = util::generate_unique_path(code);

        fs::create_dir_all(&project_path)?;

        let mut main_file_path = project_path.clone();
        main_file_path.push(self.main_file);

        let mut main_file = fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&main_file_path)?;

        main_file.write_all(code.as_bytes())?;

        Ok(project_path)
    }

    pub fn compile(&self, project_path: &Path) -> Result<()> {
        let Some(CommandArgs {
            binary: compiler,
            args,
        }) = self.compiler_args
        else {
            return Ok(());
        };

        let process = Command::new(compiler)
            .args(args)
            .current_dir(project_path)
            .stderr(Stdio::piped())
            .spawn()?;

        let compilation_error = process.wait_with_output()?.stderr;

        if !compilation_error.is_empty() {
            return Err(Error::Compilation {
                message: String::from_utf8(compilation_error)?,
            });
        }

        Ok(())
    }

    pub fn run(&self, project_path: &Path, input_path: &Path) -> Result<metrics::Metrics> {
        let output_path = project_path.join(util::hash((input_path, "output")).to_string());
        let error_path = project_path.join(util::hash((input_path, "error")).to_string());

        let sandbox = Sandbox::builder()
            .project_path(project_path)
            .config(self.sandbox_config.clone())
            .input(input_path)?
            .output_path(&output_path)
            .error_path(&error_path)
            .build();

        let sandbox = sandbox.spawn()?;
        sandbox.wait()
    }
}
