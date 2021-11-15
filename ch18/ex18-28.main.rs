// 예제 18-28 여러 패턴에 매치 가드 혼합

fn main() {
    let x = 4;
    let y = false;

    match x {
	4 | 5 | 6 if y => println!("예"),
	_ => println!("아니요"),
    }
} 
