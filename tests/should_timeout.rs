use std::time::Duration;

use code_executor::*;
use rstest::rstest;

fn get_timeout_code(language: Language<'static>) -> &'static str {
    match language {
        CPP => {
            r#"
#include <bits/stdc++.h>

using namespace std;

int main() {
    while(true) {}
}
        "#
        }
        RUST => {
            r#"
fn main() {
    loop {}
}
        "#
        }
        JAVA => {
            r#"
class Main {
    public static void main(String[] args) {
        while(true) {}
    }
}
        "#
        }
        PYTHON => {
            r#"
while True:
    continue
        "#
        }
        _ => unreachable!(),
    }
}

#[rstest]
#[tokio::test]
async fn should_timeout(#[values(CPP, RUST, JAVA, PYTHON)] language: Language<'static>) {
    let code = get_timeout_code(language);
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
