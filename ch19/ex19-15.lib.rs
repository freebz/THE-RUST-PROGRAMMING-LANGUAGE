// 예제 19-15 Millimeters 인스턴스에 Meters 인스턴스를 더하는 Add 트레이트를 Millimeters 구조체에 정의

use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
	Millimeters(self.0 + (other.0 + 1000))
    }
}
