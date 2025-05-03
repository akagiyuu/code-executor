use std::{fs, time::Duration};

use code_executor::{CommandArgs, Compiler, Language, RlimitConfig, Runner, SandboxConfig};
use nix::sys::resource::Resource;

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
        sandbox_config: SandboxConfig {
            rlimit_configs: &[
                RlimitConfig {
                    resource: Resource::RLIMIT_STACK,
                    soft_limit: 1024 * 1024 * 1024 * 1024,
                    hard_limit: 1024 * 1024 * 1024 * 1024,
                },
                RlimitConfig {
                    resource: Resource::RLIMIT_AS,
                    soft_limit: 1024 * 1024 * 1024 * 1024,
                    hard_limit: 1024 * 1024 * 1024 * 1024,
                },
                RlimitConfig {
                    resource: Resource::RLIMIT_CPU,
                    soft_limit: 60,
                    hard_limit: 90,
                },
                RlimitConfig {
                    resource: Resource::RLIMIT_FSIZE,
                    soft_limit: 1024,
                    hard_limit: 1024,
                },
            ],
            scmp_black_list: &["fork", "vfork"],
        },
    },
};

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
