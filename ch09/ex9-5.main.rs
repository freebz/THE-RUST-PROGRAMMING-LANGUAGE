// 예제 9-5 에러의 종류에 따라 다르게 처리하기

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
	Ok(file) => file,	
	Err(ref error) => match error.kind() {
	    ErrorKind::NotFound => match File::create("hello.txt") {
		Ok(fc) => fc,
		Err(e) => panic!("파일을 생성하지 못했습니다: {:?}", e),
	    },
	    other_error => panic!("파일 열기 실패: {:?}", other_error),
	},
    };
}
