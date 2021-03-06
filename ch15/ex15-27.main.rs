// 예제 15-27 자식이 없는 leaf 노드와 leaf 노드를 자식으로 갖는 branch 노드

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
	value: 3,
	children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
	value: 5,
	children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
}
