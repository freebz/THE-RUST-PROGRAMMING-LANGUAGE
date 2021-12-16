// 예제 19-20 Animal 트레이트의 baby_name 함수를 호출하는 코드. 하지만 러스트는 어떤 메서드 구현을 호출해야 하는지 알지 못함

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
    println!("새끼 강아지 이름은 {}", Animal::baby_name());
}
