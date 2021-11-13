// 예제 17-17 State 트레이트의 content 메서드를 호출하도록 수정한 Post 구조체의 content 메서드

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
	self.state.as_ref().unwrap().content(&self)
    }

    pub fn request_review(&mut self) {
	if let Some(s) = self.state.take() {
	    self.state = Some(s.request_review())
	}
    }

    pub fn approve(&mut self) {
	if let Some(s) = self.state.take() {
	    self.state = Some(s.approve())
	}
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<State>;
    fn approve(self: Box<Self>) -> Box<State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<State> {
	Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<State> {
	self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<State> {
	self
    }

    fn approve(self: Box<Self>) -> Box<State> {
	Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<State> {
	self
    }

    fn approve(self: Box<Self>) -> Box<State> {
	self
    }
}
