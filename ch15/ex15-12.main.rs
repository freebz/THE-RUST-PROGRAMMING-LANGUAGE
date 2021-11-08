// 예제 15-12 역참조 강제 덕분에 hello 함수에 MyBox<String> 값이 참조를 전달해도 정상으로 동작

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
    hello(&m);
}
