use std::time::Duration;

use code_executor::{CPP, Runner};

#[tokio::main]
async fn main() {
    let code = r#"
        #include <bits/stdc++.h>

        using namespace std;

        int main() {
            string s;
            cin >> s;
            cout << s;
        }
    "#;

    let project_path = CPP.compiler.compile(code).unwrap();
    let runner = Runner::new(
        CPP.runner_args,
        &project_path,
        Duration::from_secs(2),
        i64::MAX,
    )
    .unwrap();
    let metrics = runner.run("Hello").await.unwrap();

    println!("{:#?}", metrics);
}
