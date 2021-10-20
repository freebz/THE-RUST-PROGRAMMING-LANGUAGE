// 예제 7-13 add_to_waitlist 함수를 범위로 가져오는 코드(관용적이지 않음)

mod front_of_house {
    pub mod hosting {
	pub fn add_to_waitlist() {}
    }
}

use self::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}
