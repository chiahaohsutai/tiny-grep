use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.filepath)?;
    search(&config.query, &content, config.ignorecase)
        .iter()
        .for_each(|&ln| println!("{}", ln));
    Ok(())
}

fn search<'a>(query: &str, content: &'a str, ignorecase: bool) -> Vec<&'a str> {
    let lns = content.lines();
    if ignorecase {
        let query = &query.to_lowercase();
        lns.filter(|&s| s.to_lowercase().contains(query)).collect()
    } else { 
        lns.filter(|&s| s.contains(&query)).collect()
    }
}

pub struct Config {
    query: String,
    filepath: String,
    ignorecase: bool,
}

impl Config {
    fn new(query: String, filepath: String, ignorecase: bool) -> Self {
        Config {
            query,
            filepath,
            ignorecase,
        }
    }

    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        let query = args.get(0).ok_or("missing query parameter.")?.clone();
        let filepath = args.get(1).ok_or("missing filepath parameter.")?.clone();
        let ignorecase = env::var("IGNORECASE")
            .unwrap_or(String::from("0"))
            .to_lowercase();
        let ignorecase = ignorecase.eq("1") || ignorecase.eq("true");
        Ok(Config::new(query, filepath, ignorecase))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_sensitive_search() {
        let query = "duct";
        let content = "Rust:\nsafe, fast, productive.\nPick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, content, false)
        )
    }

    #[test]
    fn test_case_insensitive_search() {
        let query = "DuCt";
        let content = "Rust:\nsafe, fast, productive.\nPick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, content, true)
        )
    }
}
