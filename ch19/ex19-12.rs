// 예제 19-12 연관 타입 Item 필드를 갖는 Iterator 트레이트 선언

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
