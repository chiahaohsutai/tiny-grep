use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let config = tiny_grep::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Parsing error: {}", err);
        process::exit(1);
    });
    if let Err(err) = tiny_grep::run(config) {
        eprintln!("Appliction error: {}", err);
        process::exit(1);
    }
}
