// 예제 10-8 Point<T, U> 타입은 두 개의 타입으로 일반화되어 있어 필드 x와 y에 각각 다른 타입 선언

struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
