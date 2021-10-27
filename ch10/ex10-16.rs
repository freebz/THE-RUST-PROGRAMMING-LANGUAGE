// 예제 10-16 트레이트 경계에 따라 제네릭 타입의 메서드를 조건적으로 구현하기

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new (x: T, y: T) -> Self {
	Self {
	    x,
	    y,
	}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
	if self.x >= self.y {
	    println!("가장 큰 멤버는 x: {}", self.x);
	} else {
	    println!("가장 큰 멤버는 y = {}", self.y);
	}
    }
}
