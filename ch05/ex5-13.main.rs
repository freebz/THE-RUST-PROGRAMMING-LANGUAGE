// 예제 5-13 Rectangle 구조체에 메서드 정의

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
	self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    
    println!(
	"사각형의 면적: {} 제곱 픽셀",
	rect1.area()
    );
}
