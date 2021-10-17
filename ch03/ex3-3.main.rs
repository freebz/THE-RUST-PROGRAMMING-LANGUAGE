// 예제 3-3 조건식이 일치하는 동안에만 코드를 실행하는 while 루프 사용 예

fn main() {
    let mut number = 3;

    while number != 0 {
	println!("{}!", number);

	number = number - 1;
    }

    println!("발사!");
}
