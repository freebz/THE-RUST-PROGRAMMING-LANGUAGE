// 예제 19-18 fly 메서드를 명시적으로 호출

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
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
