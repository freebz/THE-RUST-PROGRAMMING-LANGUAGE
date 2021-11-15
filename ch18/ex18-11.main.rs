// 예제 18-11 섀도 변수 y를 생성하는 match 표현식

fn main() {
    let x = Some(5);
    let y = 10;

    match x {
	Some(50) => println!("50"),
	Some(y) => println!("일치, y = {:?}", y),
	_ => println!("일치하지 않음, x = {:?}", x),
    }

    println!("결과: x = {:?}, y = {:?}", x, y);
}
