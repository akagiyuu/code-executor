use std::time::Duration;

use code_executor::CPP;

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
    let metrics = CPP
        .runner
        .run(&project_path, "Hello", Duration::from_secs(1), i64::MAX)
        .await
        .unwrap();

    println!("{:#?}", metrics);
}
