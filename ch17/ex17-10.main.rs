// 예제 17-10 트레이트 객체의 트레이트를 구현하지 않는 타입 사용

use gui::Screen;

fn main() {
    let screen = Screen {
	components: vec![
	    Box::new(String::from("안녕하세요")),
	],
    };

    screen.run();
}
