// 예제 13-14 for 루프에서 반복자 활용

fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
	println!("값: {}", val);
    }
}
