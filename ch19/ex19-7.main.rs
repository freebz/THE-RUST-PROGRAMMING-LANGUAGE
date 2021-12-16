// 예제 19-7 임의의 메모리 위치를 이용해 슬라이스 생성

fn main() {
    use std::slice;

    let address = 0x01234usize;
    let r = address as *mut i32;

    let slice: &[i32] = unsafe {
	slice::from_raw_parts_mut(r, 10000)
    };
}
