// 예제 15-25 참조하는 Cons 열것값을 변경할 수 있도록 RefCell<T> 타입을 저장하는 콘스 리스트

use std::rc::Rc;
use std::cell::RefCell;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
	match *self {
	    Cons(_, ref item) => Some(item),
	    Nil => None,
	}
    }
}
