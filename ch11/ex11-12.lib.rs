// 예제 11-12 비공개 함수 테스트

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
	assert_eq!(4, internal_adder(2, 2));
    }
}
