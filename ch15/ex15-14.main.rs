// 예제 15-14 자원 해제 코드를 실행하는 Drop 트레이트를 구현한 CustomSmartPointer 구조체

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
	println!("CustomSmartPointer의 데이터 '{}'를 해제합니다!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("내 데이터") };
    let d = CustomSmartPointer { data: String::from("다른 데이터") };
    println!("CustomSmartPointer를 생성했습니다.");
}
