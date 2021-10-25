// 예제 9-10 입력값이 1에서 100 사이일 때만 인스턴스를 생성하는 Guess 타입

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
	if value < 1 || value > 100 {
	    panic!("유추한 값은 반드시 1에서 100 사이의 값이어야 합니다. 입력한 값:{}", value);
	}

	Guess {
	    value
	}
    }

    pub fn value(&self) -> i32 {
	self.value
    }
}
