// 예제 7-10 열거자를 공개하면 모든 열것값도 공개된다

mod back_of_house {
    pub enum Appetizer {
	Soup,
	Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
