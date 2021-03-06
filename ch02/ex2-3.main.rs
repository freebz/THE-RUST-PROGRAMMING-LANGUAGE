// 예제 2-3 두 숫자를 비교한 리턴값 처리

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("숫자를 맞혀봅시다!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("사용자가 맞혀야 할 숫자: {}", secret_number);

    println!("정답이라고 생각하는 숫자를 입력하세요.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
	.expect("입력한 값을 읽지 못했습니다.");

    println!("입력한 값: {}", guess);

    match guess.cmp(&secret_number) {
	Ordering::Less => println!("입력한 숫자가 작습니다!"),
	Ordering::Greater => println!("입력한 숫자가 큽니다!"),
	Ordering::Equal => println!("정답!"),
    }
}
