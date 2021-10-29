// 예제 11-7 assert_eq! 매크로를 이용해 add_two 함수 테스트

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
	assert_eq!(4, add_two(2));
    }
}
