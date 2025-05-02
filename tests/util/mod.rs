use std::{
    fs,
    path::{Path, PathBuf},
};

use code_executor::Language;

pub struct TestCase {
    pub input_path: PathBuf,
    pub output: String,
}

pub struct Problem {
    pub test_cases: Vec<TestCase>,
}

impl From<&Path> for Problem {
    fn from(problem_path: &Path) -> Self {
        let test_cases: Vec<_> = fs::read_dir(problem_path)
            .unwrap()
            .filter_map(|entry| {
                let entry = entry.unwrap();
                let path = entry.path();
                if !path.is_dir() {
                    return None;
                }

                let input_path = path.join("in.txt");

                let output_path = path.join("out.txt");
                let output = fs::read_to_string(output_path).unwrap();
                let output = output.trim().to_string();

                Some(TestCase { input_path, output })
            })
            .collect();

        Problem { test_cases }
    }
}

pub fn read_code(language: Language, problem_path: &Path) -> String {
    fs::read_to_string(problem_path.join(language.compiler.main_file)).unwrap()
}
