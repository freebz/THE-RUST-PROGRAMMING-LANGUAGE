// 예제 6-4 Coin 열거자의 Quarter 열거값에 UsState 열거자 추가

#[derive(Debug)] // 저장된 주 이름을 쉽게 디버깅할 수 있다.
enum UsState {
    Alabama, Alaska,
    // -- 생략 --
}

enum Coint {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}
