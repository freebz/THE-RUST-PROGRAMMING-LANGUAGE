// 예제 10-7 필드 x와 y는 같은 제네릭 데이터 타입 T로 선언되었으므로 반드시 같은 타입이어야 함

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 5, y: 4.0 };
}
