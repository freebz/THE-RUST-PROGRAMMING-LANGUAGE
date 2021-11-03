// 예제 13-20 Counter 구조체를 선언하고 count 필드에 0을 초깃값으로 대입해 새 인스턴스를 생성하는 new 함수

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
	Counter { count: 0 }
    }
}
