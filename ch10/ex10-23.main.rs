// 예제 10-23 각기 다른 수명을 가진 문자열값의 참조를 longest 함수에 전달

fn main() {
    let string1 = String::from("아주 아주 긴 문자열");

    {
	let string2 = String::from("xyz");
	let result = longest(string1.as_str(), string2.as_str());
	println!("더 긴 문자열: {}", result);
    }
}
