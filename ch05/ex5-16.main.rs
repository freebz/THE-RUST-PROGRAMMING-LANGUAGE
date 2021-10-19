// 예제 5-16 예제 5-15의 코드를 여러 개의 impl 블록으로 나누기

impl Rectangle {
    fn area(&self) -> u32 {
	self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
	self.width > other.width && self.height > other.height
    }
}
