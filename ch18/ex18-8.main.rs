// 예제 18-8 let 구문에 부인할 수 있는 패턴 적용

fn main() {
    let some_option_value = Some(5);
    let Some(x) = some_option_value;
}
