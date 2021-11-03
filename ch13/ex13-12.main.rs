// 예제 13-12 자신 주변에 선언된 변수를 참조하는 클로저

fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}
