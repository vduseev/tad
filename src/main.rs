mod config;
mod tests;

fn main() {
    let config = config::parse_config("tad.yml");

    // Collect input files
    tests::collect(&config.input, &config.output);

    println!("config:\n{}", config);
}
