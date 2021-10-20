// 예제 7-12 self 키워드를 이용한 상대 경로를 이용해 모듈을 범위로 가져오는 코드

mod front_of_house {
    pub mod hosting {
	pub fn add_to_waitlist() {}
    }
}

use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
