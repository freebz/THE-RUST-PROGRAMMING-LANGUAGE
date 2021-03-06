// 예제 15-16 값이 범위를 벗어나기 전에 std::mem::drop 함수를 호출해서 값 해제

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
    drop(c);
    println!("CustomSmartPointer를 main 함수의 끝에 도달하기 전에 해제합니다.");
}
