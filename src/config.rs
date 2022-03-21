use clap::{arg, Command};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub inputs: String,
    pub outputs: String,
    pub command: String,
    pub env: String,
}

pub fn get_config() -> Config {
    let matches = Command::new("tad")
        .version("0.1")
        .about("Test solutions to algorithmic and data structure problems")
        .arg(arg!([FILE]).help("Path to the source code file to run"))
        .arg(
            arg!(inputs: -i --input <GLOB>)
                .required(false)
                .multiple_occurrences(true)
                .default_value("input*.txt")
                .help("Glob pattern to look up input files"),
        )
        .arg(
            arg!(outputs: -o --output <GLOB>)
                .required(false)
                .multiple_occurrences(true)
                .default_value("output*.txt")
                .help("Glob pattern to look up output files"),
        )
        .arg(
            arg!(cmd: [COMMAND])
                .multiple_occurrences(true)
                .last(true)
                .default_value("./<FILE>")
                .help("Command to launch the program"),
        )
        .arg(
            arg!(env: -e --env <ENV>)
                .required(false)
                .default_value("ubuntu-focal")
                .help("Container environment to use"),
        )
        .arg(
            arg!(cfg: -f --file <CONFIG>)
                .required(false)
                .default_value("tad.yaml")
                .help("Path to the config file"),
        )
        .get_matches();

    println!("'config' value: {:?}", matches.value_of("cfg"));

    println!(
        "'outputs' value: {:?}",
        matches
            .values_of("outputs")
            .map(|vals| vals.collect::<Vec<_>>())
            .unwrap_or_default()
    );

    println!(
        "'inputs' value: {:?}",
        matches
            .values_of("inputs")
            .map(|vals| vals.collect::<Vec<_>>())
            .unwrap_or_default()
    );

    println!(
        "'command' values: {:?}",
        matches
            .values_of("cmd")
            .map(|vals| vals.collect::<Vec<_>>())
            .unwrap_or_default()
    );

    let config = parse_config("tad.yml");
    config
}

pub fn parse_config(filename: &str) -> Config {
    // Default config
    let default_config = Config {
        command: "a".to_string(),
        inputs: "b".to_string(),
        outputs: "c".to_string(),
        env: "e".to_string(),
    };

    let path = Path::new(filename);
    let disp = path.display();

    let contents = match fs::read_to_string(path) {
        Err(why) => {
            eprintln!("couldn't read config file {}: {}", disp, why);
            return default_config;
        }
        Ok(s) => s,
    };

    match serde_yaml::from_str(&contents) {
        Err(why) => panic!("couldn't parse config file: {}", why),
        Ok(conf) => conf,
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let padding = 12;
        write!(
            f,
            "{name:>padding$}: {value}\n",
            name = "command",
            padding = padding,
            value = self.command
        )?;
        write!(
            f,
            "{name:>padding$}: {value}\n",
            name = "input",
            padding = padding,
            value = self.inputs
        )?;
        write!(
            f,
            "{name:>padding$}: {value}\n",
            name = "output",
            padding = padding,
            value = self.outputs
        )
    }
}
