// 예제 16-12 예제의 간소화를 위해 단일 스레드 환경에서 Mutex<T>의 API 사용

use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
	let mut num = m.lock().unwrap();
	*num = 6;
    }

    println!("m = {:?}", m);
}
