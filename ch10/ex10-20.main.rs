// 예제 10-20 두 개의 문자열 슬라이스 중 길이가 긴 것을 리턴하는 longest 함수 호출

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("더 긴 문자열: {}", result);
}
