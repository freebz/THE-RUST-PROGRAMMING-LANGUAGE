// 예제 15-13 러스트가 역참조 강제를 지원하지 않았다면 작성했어야 할 코드

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
	MyBox(x)
    }
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

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
