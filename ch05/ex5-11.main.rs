// 예제 5-11 Rectangle 구조체의 인스턴스 출력

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    
    println!("rect1: {}", rect1);
}
