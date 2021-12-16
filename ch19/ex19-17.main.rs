// 예제 19-17 Human 인스턴스의 fly 메서드 호출

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
	println!("안녕하세요 기장입니다.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
	println!("날아라!");
    }
}

impl Human {
    fn fly(&self) {
	println!("*양팔을 펄떡거린다*");
    }
}

fn main() {
    let person = Human;
    person.fly();
}
