use std::fs;
use std::env;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        case_sensitive_search(&config.query, &contents)
    } else {
        case_insensitive_search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
// struct to avoid "primitive obsession"
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();

//     Config { query, filename }
// }

// Better to impl struct as its idiomatic
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            // better to return Err than to panic! so main can handle
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        // this checks an env var, note the use of .is_err here: if env var is not set this returns a True
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        println!("case_sensitive={:?}", case_sensitive);

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], case_sensitive_search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            case_insensitive_search(query, contents)
        );
    }
}

pub fn case_sensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    
    results
}

pub fn case_insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    
    results
}