// 예제 19-2 임의의 메모리에 접근하는 원시 포인터 생성

fn main() {
    let address = 0x012345usize;
    let r = address as *const i32;
}
