// 예제 17-4 Components 필드를 갖는 Screen 구조체. Components 필드에는 Draw 트레이트를 구현하는 트레이트 객체의 벡터를 저장한다.

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
