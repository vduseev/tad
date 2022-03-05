mod config;

fn main() {
    let config = config::parse_config("tad.yml");

    println!("config:\n{}", config);
}
