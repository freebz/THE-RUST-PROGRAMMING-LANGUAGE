// 예제 7-9 공개 및 비공개 필드를 모두 가진 구조체

mod back_of_house {
    pub struct Breakfast {
	pub toast: String,
	seasonal_fruit: String,
    }

    impl Breakfast {
	pub fn summer(toast: &str) -> Breakfast {
	    Breakfast {
		toast: String::from(toast),
		seasonal_fruit: String::from("복숭아"),
	    }
	}
    }
}

pub fn eat_at_restaurant() {
    // 여름에 아침 식사로 호밀빵을 주문한다.
    let mut meal = back_of_house::Breakfast::summer("호밀빵");
    // 고객이 마음을 바꿔서 빵 종류를 바꾼다.
    meal.toast = String::from("밀빵");
    println!("{} 토스트로 주세요", meal.toast);

    // 다음 줄의 주석을 해제하면 컴파일 되지 않는다.
    // 고객은 식사와 함께 제공되는 계절 과일을 선택할 수 없다.
    // mail.seasonal_fruit = String::from("블루베리");
}
