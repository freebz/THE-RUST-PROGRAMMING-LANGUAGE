// 예제 4-5 매개변수의 소유권을 다시 리턴하는 함수

fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("'{}'의 길이는 {}입니다.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 함수는 문자열의 길이를 리턴한다.

    (s, length)
}
