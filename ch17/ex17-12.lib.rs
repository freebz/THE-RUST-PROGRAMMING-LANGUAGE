// 예제 17-12 Post 구조체, new 함수, State 트레이트와 Draft 구조체 정의

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
	Post {
	    state: Some(Box::new(Draft {})),
	    content: String::new(),
	}
    }
}

trait State {}

struct Draft {}

impl State for Draft {}
