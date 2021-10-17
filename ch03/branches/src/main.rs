// 예제 3-2 if 표현식의 결과를 변수에 대입하기

fn main() {
    let condition = true;
    let number = if condition {
	5
    } else {
	6
    };

    println!("number의 값: {}", number);
}
