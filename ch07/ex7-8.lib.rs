// 예제 7-8 super로 시작하는 상대 경로로 함수 호출

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
	cook_order();
	super::serve_order();
    }

    fn cook_order() {}
}
