// 예제 7-1 front_of_house 모듈에 함수를 포함하는 하위 모듈 정의

mod front_of_house {
    mod hosting {
	fn add_to_waitlist() {}

	fn seat_at_table() {}
    }

    mod serving {
	fn take_order() {}

	fn serve_order() {}

	fn take_payment() {}
    }
}
