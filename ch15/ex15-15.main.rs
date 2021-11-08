// 예제 15-15 메모리를 일찍 해제하기 위해 Drop 트레이트의 drop 메서드를 직접 호출

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
    println!("CustomSmartPointer를 생성했습니다.");
    c.drop();
    println!("CustomSmartPointer를 main 함수의 끝에 도달하기 전에 해제합니다.");
}
