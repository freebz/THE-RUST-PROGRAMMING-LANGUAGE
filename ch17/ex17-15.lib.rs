// 예제 17-15 Post와 State 트레이트에 request_review 메서드 구현

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

    pub fn request_review(&mut self) {
	if let Some(s) = self.state.take() {
	    self.state = Some(s.request_review())
	}
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<State> {
	Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<State> {
	self
    }
}
