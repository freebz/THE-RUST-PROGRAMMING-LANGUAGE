// 예제 18-2 stack.pop() 메서드가 Some 값을 리턴하는 동안 계속해서 값을 출력하는 while let 루프

fn main() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
	println!("{}", top);
    }
}
