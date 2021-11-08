// 예제 15-7 Box<i32> 타입에 역참조 연산자 적용

fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
