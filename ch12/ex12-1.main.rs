// 예제 12-1 명령줄 인수를 수집해서 벡터로 변환 후 출력

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
