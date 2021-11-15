// 예제 18-27 매치 가드를 이용해 외부의 변수와 동등 여부 검사

fn main() {
    let x = Some(5);
    let y = 10;

    match x {
	Some(50) => println!("50"),
	Some(n) if n == y => println!("일치, n = {:?}", n),
	_ => println!("일치하지 않음, x = {:?}", x),
    }

    println!("결과: x = {:?}, y = {:?}", x, y);
} 
