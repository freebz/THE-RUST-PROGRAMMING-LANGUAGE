// 예제 16-2 thread::spawn 함수가 리턴하는 JoinHandle을 이용해 스레드 종료 기다리기

use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
	for i in 1..10 {
	    println!("새 스레드: {}", i);
	    thread::sleep(Duration::from_millis(1));
	}
    });

    for i in 1..5 {
	println!("주 스레드: {}", i);
	thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
