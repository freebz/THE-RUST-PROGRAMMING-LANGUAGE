// 예제 17-9 트레이트 객체를 이용해 같은 트레이트를 구현하는 다른 값을 저장하는 코드

use gui::{Screen, Button};

fn main() {
    let screen = Screen {
	components: vec![
	    Box::new(SelectBox {
		width: 75,
		height: 10,
		options: vec![
		    String::from("예"),
		    String::from("아니요"),
		    String::from("모름")
		],
	    }),
	    Box::new(Button {
		width: 50,
		height: 10,
		label: String::from("확인"),
	    }),
	],
    };

    screen.run();
}
