// 예제 12-4 두 번째 인수로 전달된 파일 내용 읽기

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let query = &args[1];
    let filename = &args[2];

    println!("검색어: {}", query);
    println!("대상 파일: {}", filename);

    let contents = fs::read_to_string(filename)
	.expect("파일을 읽지 못했습니다.");

    println!("파일 내용:\n{}", contents);
}
