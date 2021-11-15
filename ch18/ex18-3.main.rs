// 예제 18-3 for 루프에서 패턴을 이용해 튜플 해체

fn main() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
	println!("인덱스 {}의 값: {}", index, value);
    }
}
