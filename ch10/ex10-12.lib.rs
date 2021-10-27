// 예제 10-12 summarize 메서드가 제공하는 행위로 구성된 Summary 트레이트

pub trait Summary {
    fn summarize(&self) -> String;
}
