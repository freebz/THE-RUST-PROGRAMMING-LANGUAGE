// 예제 13-8 두 개의 서로 다른 타입에 대해 클로저를 각각 호출

fn main() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5);
}
