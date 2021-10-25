// 예제 9-9 파일을 열고 내용을 읽는 작업을 fs::read_to_string 함수로 해결한 코드

use std::io;
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
    fs:read_to_string("hello.txt")?;
}
