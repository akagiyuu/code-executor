# Code Executor

A library designed for the backend of competitive programming platforms

## Built-in Language

- C++
- Java
- Python

## Usage

- Run C++ hello world and get output

```rust
use code_executor::CPP_RUNNER;

fn main() {
    let output = CPP_RUNNER
        .run(
            r#"
                #include <bits/stdc++.h>

                using namespace std;

                int main() {
                    cout << "Hello World";
                }
            "#,
            "input.txt",
        );
    println!("{:?}", output);
}

```

- Define custom runner

```rust
let cpp_runner = Runner {
    main_file: "main.cpp",
    compiler_args: Some(CommandArgs {
        binary: "g++",
        args: &["-o", "main", "main.cpp"],
    }),
    sandbox_config: sandbox::Config {
        scmp_black_list: &[
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
        rlimit_configs: &["fork", "vfork"],
        time_limit: Duration::from_secs(2),
        args: CommandArgs {
            binary: "./main",
            args: &[],
        },
    },
};
```
