// 예제 17-7 Draw 트레이트를 구현하는 Button 구조체

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


pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
	// 실제로 버튼을 그리는 코드
    }
}
