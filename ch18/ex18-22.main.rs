// 예제 18-22 밑줄만 사용하면 값이 바인딩되지 않음

fn main() {
    let s = Some(String::from("안녕하세요"));

    if let Some(_) = s {
	println!("문자열을 찾았습니다.");
    }

    println!("{:?}", s);
}
