// 예제 16-14 Rc<T>를 이용해서 여러 스레드가 Mutex<T>의 소유권 가지기

use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
	let counter = Rc::clone(&counter);
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
