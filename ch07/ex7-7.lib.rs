// 예제 7-7 pub 키워드를 추가해서 eat_at_restaurant 함수 호출이 가능하게 변경한 코드

mod front_of_house {
    pub mod hosting {
	pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 절대 경로
    crate::front_of_house::hosting::add_to_waitlist();

    // 상대 경로
    front_of_house::hosting::add_to_waitlist();
}
