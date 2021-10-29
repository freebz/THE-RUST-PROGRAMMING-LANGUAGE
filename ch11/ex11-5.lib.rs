// 예제 11-5 제5장에서 구현했던 Rectangle 구조체와 can_hold 메서드

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
	self.length > other.length && self.width > other.width
    }
}
