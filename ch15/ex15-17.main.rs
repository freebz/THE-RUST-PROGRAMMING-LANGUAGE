// 예제 15-17 Box<T> 타입을 이용해서는 세 번째 리스트의 소유권을 공유할 수 없음을 확인

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Cons(5,
        Box::new(Cons(10,
            Box::new(Nil))));

    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}
