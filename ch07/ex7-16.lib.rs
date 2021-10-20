// 예제 7-16 as 키워드를 이용해 범위로 가져오는 타입의 이름 재정의

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // 생략
}

fn function2() -> IoResult<()> {
    // 생략
}
