// 예제 13-24 예제 12-23 Config::new 함수

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
	if args.len() < 3 {
	    return Err("필요한 인수가 지정되지 않았습니다.");
	}
	
	let query = args[1].clone();
	let filename = args[2].clone();
	let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
	Ok(Config { query, filename, case_sensitive })
    }
}
