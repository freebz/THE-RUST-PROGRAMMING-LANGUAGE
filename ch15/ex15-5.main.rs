// 예제 15-5 크기를 알고 있는 Box<T>를 이용해 다시 선언한 List 열거자

enum List {
    Cons(i32, Box<List>),
    Nil
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}
