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
        .run(&project_path, "Helo World", Duration::from_secs(1))
        .await
        .unwrap();

    println!("{:#?}", metrics);
}
