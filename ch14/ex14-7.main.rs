// 예제 14-7 adder 크레이트에서 add_one 라이브러리 크레이트 사용

use add_one;

fn main() {
    let num = 10;
    println!("안녕하세요! {} 더하기 1은 {}입니다!", num, add_one::add_one(num));
}
