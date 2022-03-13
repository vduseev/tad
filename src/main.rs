mod config;
mod test_cases;

fn main() {
    let config = config::parse_config("tad.yml");
    println!("config:\n{}", config);

    // Collect input files
    let cases = test_cases::collect_test_cases(&config.input, &config.output);

    // Iterate over cases
    for (name, case) in &cases {
        println!("{}", name);
    }
}
