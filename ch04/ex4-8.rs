// 예제 4-8 first_word 함수를 호출한 결과를 변수에 저장 후 문자열이 변경된 경우

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // 변수 word에는 5라는 값이 할당된다.

    s.clear(); // 문자열을 비워 빈 문자열("")을 만든다.

    // 변수 word에는 여전히 5라는 값이 할당되었지만, 이 값ㅇ르 적용할 문자열이 더는 없다.
    // 그래서 변수 word는 아무 쓸모가 없게 된다!
}
