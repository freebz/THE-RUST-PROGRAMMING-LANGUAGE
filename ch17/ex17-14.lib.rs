// 예제 17-14 무조건 빈 문자열 슬라이스를 리턴하도록 구현한 content 메서드

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
    
    pub fn add_text(&mut self, text: &str) {
	self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
	""
    }
}

trait State {}

struct Draft {}

impl State for Draft {}
