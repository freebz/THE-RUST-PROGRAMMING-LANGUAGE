// 예제 18-23 ..를 이용해 x 외의 나머지 필드는 무시하기

fn main() {
    struct Point {
	x: i32,
	y: i32,
	z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
	Point { x, .. } => println!("x = {}", x),
    }
} 
