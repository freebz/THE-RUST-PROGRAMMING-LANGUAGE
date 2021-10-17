// 예제 2-1 플레이어의 예측값을 입력받아 출력

use std::io;

fn main() {
    println!("숫자를 맞혀봅시다!");

    println!("정답이라고 생각하는 숫자를 입력하세요.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
	.expect("입력한 값을 읽지 못했습니다.");

    println!("입력한 값: {}", guess);
}
