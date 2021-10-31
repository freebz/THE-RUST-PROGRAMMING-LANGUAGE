// 예제 12-20 대소문자를 구분하지 않는 함수의 테스트 코드. 이 테스트는 실패함

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
	if args.len() < 3 {
	    return Err("필요한 인수가 지정되지 않았습니다.");
	}
	
	let query = args[1].clone();
	let filename = args[2].clone();
    
	Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    
    for line in search(&config.query, &contents) {
	println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines() {
	if line.contains(query) {
	    results.push(line);
	}
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
	let query = "duct";
	let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

	assert_eq!(
	    vec!["safe, fast, productive."],
	    search(query, contents)
	);
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
	    search_case_insensitive(query, contents)
	);
    }    
}
