// 예제 10-24 변수 string2가 범위를 벗어난 이후에 변수 result 사용

fn main() {
    let string1 = String::from("아주 아주 긴 문자열");
    let result;
    {
	let string2 = String::from("xyz");
	result = longest(string1.as_str(), string2.as_str());	
    }
    println!("더 긴 문자열: {}", result);
}
