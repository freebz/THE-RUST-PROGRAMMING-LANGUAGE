// 예제 7-17 pub use 키워드를 이용해 이름을 가져오는 동시에 다른 범위로도 내보내기

mod front_of_house {
    pub mod hosting {
	pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
