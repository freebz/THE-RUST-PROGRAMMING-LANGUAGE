// 예제 11-9 특정 패닉 메시지를 포함하는 패닉 발생 조건 테스트

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
	if value < 1 {
	    panic!("반드시 1보다 크거나 같은 값을 사용해야 합니다. 지정된 값: {}", value);
	} else if value > 100 {
	    panic!("반드시 100보다 작거나 같은 값을 사용해야 합니다. 지정된 값: {}", value);
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
    #[should_panic(expected = "반드시 100보다 작거나 같은 값을 사용해야 합니다.")]
    fn greater_than_100() {
	Guess::new(200);
    }
}
