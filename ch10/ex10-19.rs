// 예제 10-19 데이터가 참조보다 수명이 길어서 참조가 유효하도록 수정

{                          // -------------+-- 'b
    let x = t;		   // -+-- 'a      |
                           //  |           |
    let r = &x;            //  |           |
                           //  |           |
    println!("r: {}", r);  // -+           |
}			   // -------------+