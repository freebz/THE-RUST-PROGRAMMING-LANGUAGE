// 예제 12-18 각 줄이 검색어를 포함하는지 확인

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
    
    println!("파일 내용:\n{}", contents);

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
	if line.contains(query) {
	    // 아무 작업도 실행하지 않는다.
	}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
	let query = "duct";
	let contents = "\
Rust:
safe, fast, productive.
Pick three.";

	assert_eq!(
	    vec!["safe, fast, productive."],
	    search(query, contents)
	);
    }
}
