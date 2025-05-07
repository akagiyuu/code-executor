use std::{path::PathBuf, process::Stdio};

use tokio::{fs, io::AsyncWriteExt, process::Command};

use crate::{CommandArgs, Error, Result, util};

#[derive(Debug, Clone, Copy)]
pub struct Compiler<'a> {
    pub main_file: &'a str,
    pub args: Option<CommandArgs<'a>>,
}

impl Compiler<'_> {
    #[tracing::instrument(err)]
    async fn create_project(&self, code: &str) -> Result<PathBuf> {
        let project_path = util::generate_unique_path(code);

        fs::create_dir_all(&project_path).await?;

        let mut main_file_path = project_path.clone();
        main_file_path.push(self.main_file);

        let mut main_file = fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&main_file_path)
            .await?;

        main_file.write_all(code.as_bytes()).await?;

        Ok(project_path)
    }

    #[tracing::instrument(err)]
    pub async fn compile(&self, code: &str) -> Result<PathBuf> {
        let project_path = self.create_project(code).await?;

        let Some(CommandArgs {
            binary: compiler,
            args,
        }) = self.args
        else {
            return Ok(project_path);
        };

        let process = Command::new(compiler)
            .args(args)
            .current_dir(&project_path)
            .stderr(Stdio::piped())
            .spawn()?;

        let compilation_error = process.wait_with_output().await?.stderr;

        if !compilation_error.is_empty() {
            return Err(Error::Compilation {
                message: String::from_utf8(compilation_error)?,
            });
        }

        Ok(project_path)
    }
}
