// 예제 9-8 ? 연산자 뒤에 메서드를 연결해서 호출

use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    
    File::open("hello.txt")?.f.read_to_string(&mut s)?;
    
    Ok(s)
}
