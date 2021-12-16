// 예제 19-11 안전하지 않은 트레이트의 선언과 구현

unsafe trait Foo {
    // 메서드 선언
}

unsafe impl Foo for i32 {
    // 메서드 구현
}
