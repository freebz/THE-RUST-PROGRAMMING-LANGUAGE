// 예제 18-14 하나의 패턴에서 리터럴 값을 이용해 매칭과 해체 수행

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
	Point { x, y: 0 } => println!("x 축 {}에 위치하는 점", x),
	Point { x: 0, y } => println!("y 축 {}에 위치하는 점", y),
	Point { x, y } => println!("좌표 ({}, {})에 위치하는 점", x, y),
    }
}
