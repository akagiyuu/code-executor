use std::time::Duration;

use code_executor::{CommandArgs, Compiler, Language, Runner};

pub const CPP: Language = Language {
    compiler: Compiler {
        main_file: "main.cpp",
        args: Some(CommandArgs {
            binary: "g++",
            args: &["-o", "main", "main.cpp"],
        }),
    },
    runner_args: CommandArgs {
        binary: "./main",
        args: &[],
    },
};

#[tokio::main]
async fn main() {
    let code = br#"
        #include <bits/stdc++.h>

        using namespace std;

        int main() {
            string s;
            cin >> s;
            cout << s;
        }
    "#;

    let project_path = CPP.compiler.compile(code).await.unwrap();
    let runner = Runner::new(
        CPP.runner_args,
        &project_path,
        Duration::from_secs(2),
        i64::MAX,
        512,
    )
    .unwrap();
    let metrics = runner.run(b"Hello").await.unwrap();

    println!("{:#?}", metrics);
}
