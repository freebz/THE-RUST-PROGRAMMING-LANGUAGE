// 예제 6-6 주어진 값이 Some(3)일 때 처리

let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
