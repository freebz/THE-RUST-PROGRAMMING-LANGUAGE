// 예제 6-3 열거자와 각 값에 해당하는 패턴을 match 표현식으로 구현

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
	Coin::Penny => 1,
	Coin::Nickle => 5,
	Coin::Dime => 10,
	Coin::Quarter => 25,
    }
}
