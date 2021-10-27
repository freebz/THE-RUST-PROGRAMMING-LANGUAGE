// 예제 10-22 함수 시그너처의 모든 참조들이 같은 수명 'a를 가져야 한다는 것을 명시하는 longest 함수 선언

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
	x
    } else {
	y
    }
}
