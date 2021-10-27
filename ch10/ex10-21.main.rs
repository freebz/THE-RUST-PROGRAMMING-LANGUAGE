// 예제 10-21 두 개의 문자열 슬라이스 중 길이가 긴 것을 리턴하는 longest 함수, 컴파일되지 않음

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
	x
    } else {
	y
    }
}
