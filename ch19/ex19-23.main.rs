// 예제 19-23 Display 트레이트를 구현하기 위해 Vec<String> 타입을 감싸는 Wrapper 타입 구현

use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("안녕하세요"),
			 String::from("러스트입니다.")]);
    println!("w = {}", w);
}
