// 예제 13-5 클로저를 선언하고 expensive_closure 변수에 저장

let expensive_closure = |num| {
    println!("시간이 오래 걸리는 계산을 수행 중···");
    thread::sleep(Duration::from_secs(2));
    num
};
