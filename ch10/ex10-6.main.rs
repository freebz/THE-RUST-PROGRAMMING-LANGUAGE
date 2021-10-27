// 예제 10-6 T 타입의 필드 x와 y를 정의하는 Point<T> 구조체

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
