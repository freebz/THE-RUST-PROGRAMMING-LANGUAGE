// 예제 19-25 중복을 제거하기 위해 Thunk 타입 별칭 사용

type Thunk = Box<dyn Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("안녕하세요"));

fn takes_long_type(f: Thunk) {
    // -- 생략 --
}

fn returns_long_type() -> Thunk {
    // -- 생략 --
}
