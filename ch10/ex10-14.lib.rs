// 예제 10-14 summarize 메서드의 기본 구현을 제공하는 Summary 트레이트

pub trait Summary {
    fn summarize(&self) -> String {
	String::from("(계속 읽기)")
    }
}
