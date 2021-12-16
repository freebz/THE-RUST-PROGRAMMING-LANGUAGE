// 예제 19-24 긴 이름의 타입 사용하기

let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("안녕하세요"));

fn takes_long_type(f: Box<Fn() + Send + 'static>) {
    // -- 생략 --
}

fn returns_long_type() -> Box<Fn() + Send + 'static> {
    // -- 생략 --
}
