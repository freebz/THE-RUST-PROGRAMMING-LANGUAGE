// 예제 19-30 예제 크레이트가 제공하는 절차적 매크로 사용

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
