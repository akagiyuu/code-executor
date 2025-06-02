mod util;

use std::{path::Path, time::Duration};

use bstr::ByteSlice;
use code_executor::*;
use rstest::rstest;

use util::read_code;

#[rstest]
#[tokio::test]
async fn should_timeout(#[values(CPP, RUST, JAVA, PYTHON)] language: Language<'static>) {
    let code = read_code(language, Path::new("tests/data/timeout"));
    let project_path = language.compiler.compile(code.as_bytes()).await.unwrap();

    let runner = Runner::new(
        language.runner_args,
        &project_path,
        Duration::from_secs(2),
        i64::MAX,
        512,
    )
    .unwrap();

    let metrics = runner.run(b"").await.unwrap();

    assert_eq!(metrics.exit_status, ExitStatus::Timeout)
}
