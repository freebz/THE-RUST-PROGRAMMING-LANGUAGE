// 예제 18-17 함수 시그너처에서 밑줄 사용

fn foo(_: i32, y: i32) {
    println!("이 함수는 y 매개변수만 사용한다: {}", y);
}

fn main() {
    foo(3, 4);
}
