// 예제 19-9 불변 정적 변수를 선언 및 사용

static HELLO_WORLD: &str = "안녕하세요!";

fn main() {
    println!("인삿말: {}", HELLO_WORLD);
}
