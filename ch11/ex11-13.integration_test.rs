// 예제 11-13 adder 크레이트 안의 함수를 테스트하는 통합 테스트

use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
