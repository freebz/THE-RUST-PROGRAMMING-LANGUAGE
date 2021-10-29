// 예제 11-8 패닉 유발 상황 테스트

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
	if value < 1 || value > 100 {
	    panic!("반드시 1과 100 사이의 값을 사용해야 합니다. 지정된 값: {}", value);
	}

	Guess {
	    value
	}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
	Guess::new(200);
    }
}
