use std::env;
use std::fs;
use std::error::Error;
use toml::Value;

pub struct Config {
    // pub query: String,
    // pub file_path: String,
    // pub ignore_case: bool,
    pub ip_address: String,
    pub monitor: bool,
}


impl Config {
    pub fn build(
        mut args: env::Args,
    ) -> Result<Config, Box<dyn Error>> {
        let config_file = fs::read_to_string("./config.toml")?;
        let config: Value = toml::from_str(&config_file)?;

        let network = config.get("network").and_then(|v| v.as_table()).ok_or("Missing [network] table")?;
        let ip_address = network.get("ip_address").and_then(|v| v.as_str()).ok_or("Missing ip_address field")?.to_string();


        args.next();

        let ip_address = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an IP address".to_string()),
        };

        // let query = match args.next() {
        //     Some(arg) => arg,
        //     None => return Err("Didn't get a query string"),
        // };
        // let config_file = fs::read_to_string(dbg!("./config.toml"))?;

        let mut monitor = false;
        while let Some(arg) = args.next() {
            if arg == "--monitor=on" {
                monitor = true;
            } 
            if arg == "--monitor=off" {
                monitor = false;
            } 
            // else {
            //     return Err(format!("Invalid argument: {}", arg));
            // }
        }
        // let file_path = match args.next() {
        //     Some(arg) => arg,
        //     None => return Err("Didn't get a file path"),
        // };

        // let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            ip_address,
            monitor,
        })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    // let contents = fs::read_to_string(dbg!(config.file_path))?;

    // let results = if config.ignore_case {
    //     search_case_insensitive(&config.query, &contents)
    // } else {
    //     search(&config.query, &contents)
    // };

    // for line in results {
    //     println!("{line}");
    // }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
