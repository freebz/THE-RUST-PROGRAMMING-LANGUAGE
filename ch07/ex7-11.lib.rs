// 예제 7-11 use 키워드로 모듈을 범위로 가져오는 코드

mod front_of_house {
    pub mod hosting {
	pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
