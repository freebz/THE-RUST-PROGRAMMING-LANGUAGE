// 예제 19-14 Add 트레이트를 구현해서 Point 인스턴스에 대한 + 연산자 기능 오버로딩

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
	Point {
	    x: self.x + other.x,
	    y: self.y + other.y,
	}
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
	       Point { x: 3, y: 3 });
}
