// 예제 10-10 제네릭 타입 매개션수 T가 특정 구체화된 타입일 때만 적용되는 impl 블록

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
	(self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
