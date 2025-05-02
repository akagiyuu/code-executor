mod util;

use std::{path::PathBuf, time::Duration};

use code_executor::*;
use rstest::rstest;
use util::{Problem, read_code};

#[rstest]
fn should_output_correct(
    #[values(CPP, RUST, JAVA, PYTHON)] language: Language,
    #[files("tests/problem/*")] problem_path: PathBuf,
) {
    let problem: Problem = problem_path.as_path().into();

    let code = read_code(language, &problem_path);
    let project_path = language.compiler.compile(&code).unwrap();

    for test_case in problem.test_cases {
        let metrics = language.runner.run(
            &project_path,
            &test_case.input_path,
            Duration::from_secs(10),
        ).unwrap();
        assert_eq!(metrics.output, test_case.output);
    }
}
