// 예제 15-11 &str 타입의 매개변수 name을 사용하는 hello 함수

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


use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
	&self.0
    }
}

fn hello(name: &str) {
    println!("안녕하세요 {}!", name);
}
