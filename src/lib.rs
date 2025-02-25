use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.filepath)?;
    Ok(println!("{}", content))
}

pub struct Config {
    query: String,
    filepath: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        let query = args.get(0).ok_or("missing query parameter.")?.clone();
        let filepath = args.get(1).ok_or("missing filepath parameter.")?.clone();
        Ok(Config { query, filepath })
    }
}
