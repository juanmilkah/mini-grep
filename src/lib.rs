use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

pub struct Config {
    needle: String,
    haystack: Vec<String>,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments;");
        }
        let needle = args[1].clone();
        let mut haystack = vec![args[2].clone()];
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        if haystack.contains(&'*'.to_string()) {
            /*pop to remove the asterick character*/
            let _ = haystack.pop();
            // Here we implement the recursive search in the current directory.
            let paths = std::fs::read_dir("./").unwrap();

            for path in paths {
                let path = path.unwrap().path();
                if path.is_file() {
                    haystack.push(path.display().to_string());
                }
            }
        }
        Ok(Config {
            needle,
            haystack,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<Vec<String>, Box<dyn Error>> {
    let mut matches = 0;
    let mut results: Vec<String> = vec![];
    println!("File Name;\tLine Number;\tLine Text;");
    for file_path in config.haystack {
        let file = File::open(&file_path)?;
        let reader = io::BufReader::new(file);

        for (index, line) in reader.lines().enumerate() {
            let line = line?;
            let is_match = if config.ignore_case {
                line.to_lowercase().contains(&config.needle.to_lowercase())
            } else {
                line.contains(&config.needle)
            };

            if is_match {
                matches += 1;
                println!("{}:\tLine {}:\t\t{}", file_path, index + 1, line);
                results.push(line);
            }
        }
    }

    println!(
        "\n{} Match{} Found",
        matches,
        if matches == 1 { "" } else { "es" }
    );
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let needle = "Hello".to_string();
        let haystack = vec!["hello".to_string()];
        let ignore_case = true;
        let conf = Config {
            needle,
            haystack,
            ignore_case,
        };

        assert_eq!(
            run(conf).unwrap(),
            vec!["hello from the terminal;", "Hello;"]
        );
    }

    #[test]
    fn two_result() {
        let needle = "Hello".to_string();
        let haystack = vec!["hello".to_string()];
        let ignore_case = false;
        let conf = Config {
            needle,
            haystack,
            ignore_case,
        };

        assert_eq!(run(conf).unwrap(), vec!["Hello;"]);
    }

    #[test]
    fn wildcard_search() {
        let needle = "hello".to_string();
        let haystack = vec!["./hello".to_string(), "./lorem".to_string()];
        let ignore_case = false;
        let conf = Config {
            needle,
            haystack,
            ignore_case,
        };
        assert_eq!(
            run(conf).unwrap(),
            vec!["hello from the terminal;", "hello from rust world!"]
        );
    }
}
