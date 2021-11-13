// 예제 17-20 PendingReviewPost 인스턴스를 생성하는 DraftPost의 request_review 메서드와 발행된 Post 인스턴스를 생성하는 PendingReviewPost 구조체의 approve 메서드

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

    pub fn request_review(self) -> PendingReviewPost {
	PendingReviewPost {
	    content: self.content,
	}
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
	Post {
	    content: self.content,
	}
    }
}
