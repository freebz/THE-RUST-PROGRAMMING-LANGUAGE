// 예제 15-6 역참조 연산자를 이용해 i32 값의 참조 따라가기

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
