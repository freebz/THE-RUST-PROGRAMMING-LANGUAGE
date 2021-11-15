// 예제 18-26 패턴에 매치 가드 추가

fn main() {
    let num = Some(4);

    match num {
	Some(x) if x < 5 => println!("5보다 작은 값: {}", x),
	Some(x) => println!("{}", x),
	None => (),
    }
} 
