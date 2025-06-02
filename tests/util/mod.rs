use std::{fs, path::Path};

use code_executor::Language;

#[allow(unused)]
pub struct TestCase {
    pub input: Vec<u8>,
    pub output: Vec<u8>,
}

#[allow(unused)]
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
                let input = fs::read(input_path).unwrap();

                let output_path = path.join("out.txt");
                let output = fs::read(output_path).unwrap();

                Some(TestCase { input, output })
            })
            .collect();

        Problem { test_cases }
    }
}

#[allow(unused)]
pub fn read_code(language: Language, problem_path: &Path) -> Vec<u8> {
    fs::read(problem_path.join(language.compiler.main_file)).unwrap()
}
