// 예제 19-16 fly 메서드를 이미 선언한 Human 구조체에 각각 fly 메서드를 정의하는 두 트레이트 구현

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
