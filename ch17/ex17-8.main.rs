// 예제 17-8 gui 라이브러리를 이용해 Draw 트레이트를 구현하는 SelectBox 구조체

use gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
	// SelectBox를 그리는 코드
    }
}
