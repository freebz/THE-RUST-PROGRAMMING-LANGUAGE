// 예제 9-9 파일을 열고 내용을 읽는 작업을 fs::read_to_string 함수로 해결한 코드

use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
