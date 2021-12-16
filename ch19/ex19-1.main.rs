// 예제 19-1 참조에서 원시 포인터 생성

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
}
