// 예제 15-3 List 열거자를 이용해 1, 2, 3 목록 저장

enum List {
    Cons(i32, List),
    Nil
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
