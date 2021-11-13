// 예제 17-6 제네릭과 트레이트 경계를 이용해 구현한 Screen 구조체와 run 메서드

pub trait Draw {
    fn draw(&self);
}

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
    where T: Draw {
    pub fn run(&self) {
	for component in self.components.iter() {
	    component.draw();
	}
    }
}
