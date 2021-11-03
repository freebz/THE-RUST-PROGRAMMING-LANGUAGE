// 예제 13-25 env::args 함수의 리턴값을 Config::new 함수에 그대로 전달

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
	eprintln!("인수를 구문분석하는 동안 오류가 발생했습니다: {}", err);
	process::exit(1);
    });
    
    if let Err(e) = minigrep::run(config) {
	eprintln!("애플리케이션 에러: {}", e);

	process::exit(1);
    }
}
