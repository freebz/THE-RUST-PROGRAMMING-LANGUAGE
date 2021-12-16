// 예제 19-8 다른 언어에 선언된 외부 함수 선언 및 호출

extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
	println!("C 언어에 따르면 -3의 절대값은 {}입니다.", abs(-3));
    }
}
