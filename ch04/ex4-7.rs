// 예제 4-7 first_world 함수는 전달된 문자열 매개변수에서 바이트의 인덱스값 리턴

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
	if item == b' ' {
	    return i;
	}
    }
    s.len()
}
