// 예제 16-1 주 스레드와 새 스레드에서 각각 메시지 출력

use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
	for i in 1..10 {
	    println!("새 스레드: {}", i);
	    thread::sleep(Duration::from_millis(1));
	}
    });

    for i in 1..5 {
	println!("주 스레드: {}", i);
	thread::sleep(Duration::from_millis(1));
    }
}
