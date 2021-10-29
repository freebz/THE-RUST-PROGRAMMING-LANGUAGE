// 예제 11-10 println! 매크로 호출 함수 테스트

fn prints_and_returns_10(a: i32) -> i32 {
    println!("입력값: {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
	let value = prints_and_returns_10(4);
	assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
	let value = prints_and_returns_10(8);
	assert_eq!(5, value);
    }
}
