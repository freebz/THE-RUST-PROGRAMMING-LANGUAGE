// 예제 19-3 unsafe 블록 안에서 원시 포인터 역참조

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
	println!("r1 = {}", *r1);
	println!("r2 = {}", *r2);
    }
}
