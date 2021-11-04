// 예제 14-6 다시 노출된 아이템을 사용하는 프로그램

use art::PrimaryColor;
use art::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
