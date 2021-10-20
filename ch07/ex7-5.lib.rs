// 예제 7-5 eat_at_restaurant 함수가 접근할 수 있도록 hosting 모듈에 pub 키워드 추가

mod front_of_house {
    pub mod hosting {
	fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 절대 경로
    crate::front_of_house::hosting::add_to_waitlist();

    // 상대 경로
    front_of_house::hosting::add_to_waitlist();
}
