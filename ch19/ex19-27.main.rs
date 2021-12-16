// 예제 19-27 fn 타입을 이용해 함수 포인터를 인수로 사용하는 함수

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("정답은 {}", answer);
}
