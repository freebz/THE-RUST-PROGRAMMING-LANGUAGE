// 예제 3-4 while 루프를 이용해 컬렉션 내 각 요소 반복 처리

fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
	println!("요소의 값: {}", a[index]);

	index = index + 1;
    }
}
