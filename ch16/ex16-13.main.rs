// 예제 16-13 Mutex<T>가 보호하는 counter 값을 증가시키는 10개의 스레드 생성

use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
	let handle = thread::spawn(move || {
	    let mut num = counter.lock().unwrap();

	    *num += 1;
	});
	handles.push(handle);
    }

    for handle in handles {
	handle.join().unwrap();
    }
    
    println!("결과: {}", *counter.lock().unwrap());
}
