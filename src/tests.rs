use glob::glob;

pub fn collect(input: &str, output: &str) {
    let inputs = glob(input).expect("Failed to read input glob pattern");
    let outputs = glob(output).expect("Failed to read output glob pattern");

    for i in inputs {
        match i {
            Ok(path) => println!("input: {}", path.display()),
            Err(why) => println!("failed input: {}", why),
        }
    }

    for o in outputs {
        match o {
            Ok(path) => println!("output: {}", path.display()),
            Err(why) => println!("failed output: {}", why),
        }
    }
}

// fn extract_name(pattern: &str, path: &str) {}
