// Pwp 12-13 Config 타입과 run 함수를 src/lib.rs 파일로 옮김

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
