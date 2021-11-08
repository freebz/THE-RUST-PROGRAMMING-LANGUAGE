// 예제 15-9 참조와 Box<T> 타입을 사용했던 것과 같은 방법으로 MyBox<T> 타입 사용

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
	MyBox(x)
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
