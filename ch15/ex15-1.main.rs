// 예제 15-1 박스를 이용해 i32값을 힙 메모리에 저장

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
