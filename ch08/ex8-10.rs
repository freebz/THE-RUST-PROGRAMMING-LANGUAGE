// 예제 8-10 열거자를 이용해 하나의 벡터에 다른 타입의 값 저장

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("블루")),
    SpreadsheetCell::Float(10.12),
];
