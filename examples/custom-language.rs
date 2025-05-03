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
    runner: Runner {
        args: CommandArgs {
            binary: "./main",
            args: &[],
        },
        max_memory: 1024 * 1024 * 1024,
        max_cpu_percentage: 50,
    },
};

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
        .run(&project_path, "Hello World", Duration::from_secs(1))
        .await
        .unwrap();

    println!("{:#?}", metrics);
}
