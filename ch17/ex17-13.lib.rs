// 예제 17-13 포스트의 콘텐츠에 텍스트를 추가하는 add_text 메서드

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
}

trait State {}

struct Draft {}

impl State for Draft {}
