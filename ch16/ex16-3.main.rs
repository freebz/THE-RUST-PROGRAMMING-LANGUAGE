// 예제 16-3 주 스레드가 생성한 벡터를 다른 스레드에서 사용

use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    
    let handle = thread::spawn(|| {
	println!("백터 {:?}", v);
    });

    handle.join().unwrap();
}
