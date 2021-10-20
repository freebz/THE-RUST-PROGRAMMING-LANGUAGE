// 예제 7-3 절대 및 상대 경로를 이용해 add_to_waitlist 함수 호출

mod front_of_house {
    mod hosting {
	fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 절대 경로
    crate::front_of_house::hosting::add_to_waitlist();

    // 상대 경로
    front_of_house::hosting::add_to_waitlist();
}
