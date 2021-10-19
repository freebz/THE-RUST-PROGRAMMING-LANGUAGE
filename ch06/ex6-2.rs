// 예제 6-2 개발 값을 각기 다른 타입으로 정의한 Message 열거자

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
