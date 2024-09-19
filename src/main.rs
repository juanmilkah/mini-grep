use mini_grep::{run, Config};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(_) = run(config) {
        eprintln!("Cannot Read from File;");
        process::exit(1);
    }
}
