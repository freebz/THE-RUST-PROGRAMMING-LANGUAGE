// 예제 9-6 match 표현식을 이용해 호출자에게 에러를 리턴하는 함수

use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
	Ok(file) => file,
	Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
	Ok(_) => Ok(s),
	Err(e) => Err(e),
    }
}
