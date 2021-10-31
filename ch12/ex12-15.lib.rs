// 예제 12-15 search 함수에 대한 실패 테스트 생성

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
