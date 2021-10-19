// 예제 5-15 Rectangle 구조체에 추가한 can_hold 메서드. 이 메서드는 또 다른 Rectangle 구조체의 인스턴스를 매개변수로 사용

impl Rectangle {
    fn area(&self) -> u32 {
	self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
	self.width > other.width && self.height > other.height
    }
}
