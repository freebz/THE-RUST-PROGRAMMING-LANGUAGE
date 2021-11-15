// 예제 18-21 밑줄로 시작하는 사용하지 않는 변수는 여전히 값을 바인딩하며 값의 소유권이 있음

fn main() {
    let s = Some(String::from("안녕하세요"));

    if let Some(_s) = s {
	println!("문자열을 찾았습니다.");
    }

    println!("{:?}", s);
}
