// 예제 11-6 면적이 큰 사각형이 면적이 작은 사각형을 포함하는지 확인하는 can_hold 메서드 테스트(예제 11-5에 이어서 작성)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
	let larger = Rectangle { length: 8, width: 7 };
	let smaller = Rectangle { length: 5, width: 1 };

	assert!(larger.can_hold(&smaller));
    }
}
