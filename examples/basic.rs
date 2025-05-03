use std::{fs, time::Duration};

use code_executor::CPP;

fn main() {
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
        .run(
            &project_path,
            &fs::canonicalize("examples/input.txt").unwrap(),
            Duration::from_secs(1),
        )
        .unwrap();

    println!("{:#?}", metrics);
}
