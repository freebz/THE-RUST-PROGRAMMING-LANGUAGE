// 예제 16-5 move 키워드를 이용해서 클로저에 값의 소유권을 이전

use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    
    let handle = thread::spawn(move || {
	println!("백터 {:?}", v);
    });

    handle.join().unwrap();
}
