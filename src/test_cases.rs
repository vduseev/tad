use std::collections::HashMap;
use std::path::{Path, PathBuf};

use difference::{Changeset, Difference};
use glob::{glob, Paths};

pub struct TestCase {
    input: PathBuf,
    output: Option<PathBuf>,
}

/// Collects test cases based on given glob patterns for input and output.
///
/// # Arguments
///
/// * `input` - Glob pattern of input files.
/// * `output` - Glob pattern of output files.
///
pub fn collect_test_cases(input: &str, output: &str) -> HashMap<String, TestCase> {
    let inputs: Paths = glob(input).expect("Failed to read input glob pattern");
    let outputs: Paths = glob(output).expect("Failed to read output glob pattern");

    let mut test_cases: HashMap<String, TestCase> = HashMap::new();

    for i in inputs {
        match i {
            Ok(path) => {
                let name = extract_name(input, &path);
                let test_case = TestCase {
                    input: path,
                    output: None,
                };

                test_cases.insert(name, test_case);
            }
            Err(why) => println!("failed to produce input path from glob: {}", why),
        }
    }

    for o in outputs {
        match o {
            Ok(path) => {
                let name = extract_name(output, &path);
                if test_cases.contains_key(&name) {
                    let tc: Option<&mut TestCase> = test_cases.get_mut(&name);
                    tc.unwrap().output = Some(path);
                }
            }
            Err(why) => println!("failed to produce output path from glob: {}", why),
        }
    }

    test_cases
}

/// Extracts a unique name of test file based on how it differs from the given glob pattern.
///
/// # Arguments
///
/// * `pattern` - Glob pattern that produced this file path.
/// * `path` - File path to compare with the glob pattern.
///
fn extract_name(pattern: &str, path: &Path) -> std::string::String {
    // We will gradually build the name by concatenating "Add" parts of the diff changeset.
    let mut name = "".to_owned();

    // Compute diff changeset using the difference library.
    let changeset = Changeset::new(pattern, &path.to_string_lossy(), "");
    for d in changeset.diffs.iter() {
        match d {
            Difference::Add(s) => {
                name.push_str(s);
            }
            _ => {}
        }
    }

    return name;
}
