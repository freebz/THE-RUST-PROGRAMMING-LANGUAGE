// 예제 10-26 예제 4-9에서 선언했던 함수. 매개변수와 리턴 타입이 참조임에도 수명 애노테이션이 없는데 컴파일됨

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
	if item == b' ' {
	    return &s[0..i];
	}
    }

    &s[..]
}
