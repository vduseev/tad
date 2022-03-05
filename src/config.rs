use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub command: String,
    pub input: String,
    pub output: String,
}

pub fn parse_config(filename: &str) -> Config {
    // Default
    let default_config = Config {
        command: "a".to_string(),
        input: "b".to_string(),
        output: "c".to_string(),
    };

    // Open file
    let path = Path::new(filename);
    let disp = path.display();
    // let file = match File::open(&path) {
    //     Err(why) => panic!("couldn't open config file {}: {}", disp, why),
    //     Ok(file) => file,
    // };

    // let mut contents = String::new();
    // match file.read_to_string(&mut contents) {
    //     Err(why) => panic!("couldn't read config file {}: {}", disp, why),
    //     Ok(_) => println!("Parsed config file {}", disp),
    // };

    let contents = match fs::read_to_string(path) {
        // Err(why) => panic!(),
        Err(why) => {
            eprintln!("couldn't read config file {}: {}", disp, why);
            return default_config;
        }
        Ok(s) => s,
    };

    // let s = contents.as_str();

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
            value = self.input
        )?;
        write!(
            f,
            "{name:>padding$}: {value}\n",
            name = "output",
            padding = padding,
            value = self.output
        )
    }
}
