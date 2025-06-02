use std::{path::PathBuf, process::Stdio};

use tokio::{fs, io::AsyncWriteExt, process::Command};

use crate::{CommandArgs, Error, Result, util};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Compiler<'a> {
    pub main_file: &'a str,
    pub args: Option<CommandArgs<'a>>,
}

impl Compiler<'_> {
    #[tracing::instrument(err)]
    async fn create_project(&self, code: &[u8]) -> Result<PathBuf> {
        let project_path = util::generate_unique_path();

        fs::create_dir_all(&project_path).await?;

        let mut main_file_path = project_path.clone();
        main_file_path.push(self.main_file);

        let mut main_file = fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&main_file_path)
            .await?;

        main_file.write_all(code).await?;

        Ok(project_path)
    }

    #[tracing::instrument(err)]
    pub async fn compile(&self, code: &[u8]) -> Result<PathBuf> {
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

#[cfg(test)]
mod test {
    use std::
        path::Path
    ;

    use bstr::ByteSlice;
    use rstest::rstest;

    use crate::{CPP, JAVA, Language, PYTHON, RUST, test::read_code};

    const EXAMPLE_CODE_DIR: &str = "tests/data/timeout";

    #[rstest]
    #[tokio::test]
    async fn should_create_valid_project_directory(
        #[values(CPP, RUST, JAVA, PYTHON)] language: Language<'static>,
    ) {
        let code = read_code(language, Path::new(EXAMPLE_CODE_DIR));
        let project_path = language.compiler.create_project(code.as_bytes()).await.unwrap();
        let main_path = project_path.join(language.compiler.main_file);

        assert!(main_path.exists())
    }

    #[rstest]
    #[tokio::test]
    async fn should_compile_successfully(
        #[values(CPP, RUST, JAVA, PYTHON)] language: Language<'static>,
    ) {
        let code = read_code(language, Path::new(EXAMPLE_CODE_DIR));
        assert!(language.compiler.compile(code.as_bytes()).await.ok());
    }
}
