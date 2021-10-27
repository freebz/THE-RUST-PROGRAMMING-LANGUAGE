// 예제 10-9 Point<T> 구조체에 T 타입의 필드 x의 참조를 리턴하는 메서드 x 정의

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
	&self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
