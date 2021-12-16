// 예제 19-21 완전한 식별자 문법을 이용해서 Dog 구조체가 구현하는 Animal 트레이트의 baby_name 함수 호출

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
    println!("새끼 강아지 이름은 {}", <Dog as Animal>::baby_name());
}
