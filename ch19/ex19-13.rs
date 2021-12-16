// 예제 19-13 제네릭을 이용해 선언한 가상의 Iterator 트레이트

pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
