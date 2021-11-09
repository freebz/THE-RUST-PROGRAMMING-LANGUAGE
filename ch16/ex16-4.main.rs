// 예제 16-4 참조를 해제하는 주 스레드로부터 v에 대한 참조를 캡처하려는 클로저를 실행하는 스레드

use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    
    let handle = thread::spawn(|| {
	println!("백터 {:?}", v);
    });

    drop(v); // 어이쿠!

    handle.join().unwrap();
}
