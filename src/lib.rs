use std::error::Error;
use std::fs;

pub struct Config {
    needle: String,
    haystack: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments;");
        }
        let needle = args[1].clone();
        let haystack = args[2].clone();

        Ok(Config { needle, haystack })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let haystack = fs::read_to_string(config.haystack)?;

    for line in search(&config.needle, &haystack) {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(needle: &str, haystack: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in haystack.lines() {
        if line.contains(needle) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let needle = "Hello";
        let haystack = "Hello world;\nWelcome to Elclassico";

        assert_eq!(search(needle, haystack), vec!["Hello world;"]);
    }
}
