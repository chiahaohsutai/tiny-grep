use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.filepath)?;
    search(&config.query, &content).iter().for_each(|&ln| println!("{}", ln));
    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines().filter(|&s| s.contains(query)).collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_result_search() {
        let query = "duct";
        let content = "Rust:\nsafe, fast, productive.\nPick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content))
    }
}
