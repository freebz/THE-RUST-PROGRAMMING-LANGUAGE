// 예제 17-5 각 컴포넌트의 draw 메서드를 호출하는 Screen 구조체의 run 메서드

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
	for component in self.components.iter() {
	    component.draw();
	}
    }
}
