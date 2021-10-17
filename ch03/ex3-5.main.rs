// 예제 3-5 for 루프를 이용해 컬렉션의 각 요소 반복 처리

fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
	println!("요소의 값: {}", element);
    }
}
