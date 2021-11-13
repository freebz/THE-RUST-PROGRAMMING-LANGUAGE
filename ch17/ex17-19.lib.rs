// 예제 17-19 content 메서드를 정의한 Post 구조체와 content 메서드를 구현하지 않은 DraftPost 구조체

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
	DraftPost {
	    content: String::new(),
	}
    }

    pub fn content(&self) -> &str {
	&self.content
    }
}

impl DraftPost {    
    pub fn add_text(&mut self, text: &str) {
	self.content.push_str(text);
    }
}
