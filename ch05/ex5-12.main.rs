// 예제 5-12 Rectangle 구조체가 디버깅 정보를 출력할 수 있도록 Debug 트레이트를 상속하는 애노테이션 추가

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    
    println!("rect1: {:?}", rect1);
}
