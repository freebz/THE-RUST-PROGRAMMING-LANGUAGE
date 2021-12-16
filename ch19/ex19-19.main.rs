// 예제 19-19 구현하는 트레이트의 연관 함수와 같은 이름을 가진 구조체

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
	String::from("점박이")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
	String::from("멍멍이")
    }
}

fn main() {
    println!("새끼 강아지 이름은 {}", Dog::baby_name());
}
