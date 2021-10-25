// 예제 9-4 match 표현식을 이용해 리턴된 Result 타입의 리턴값 처리

use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
	Ok(file) => file,
	Err(error) => {
	    panic!("파일 열기 실패: {:?}", error);
	}
    };
}
