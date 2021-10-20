// 예제 7-15 서로 다른 부모 모듈에 적의된 같은 이름을 가진 두 타입을 현재 범위로 가져오는 예제

use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // 생략
}

fn function2() -> io::Result<()> {
    // 생략
}
