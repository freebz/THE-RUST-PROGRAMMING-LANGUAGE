// 예제 18-9 부인할 수 있는 패턴을 적용하기 위해 let 구문을 if let 블록으로 수정

fn main() {
    let some_option_value = Some(5);
    if let Some(x) = some_option_value {
	println!("{}", x);
    }
}
