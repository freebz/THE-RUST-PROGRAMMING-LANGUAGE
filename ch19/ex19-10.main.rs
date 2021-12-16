// 예제 19-10 가변 정적 변수의 값을 읽거나 쓰는 작업은 안전하지 않음

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
	COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
	println!("COUNTER: {}", COUNTER);
    }
}
