// 예제 14-4 art 라이브러리의 내부 구조에 따라 필요한 항목 사용

use art::kinds::PrimaryColor;
use art::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
