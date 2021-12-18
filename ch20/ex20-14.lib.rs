// 예제 20-14 ThreadPool 구조체에 스레드를 저장하는 벡터 추가

use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// 새 ThreadPool 인스턴스를 생성한다.
    ///
    /// size 매개변수는 풀의 스레드 개수를 지정한다.
    ///
    /// # Panics
    ///
    /// size 매개변수의 값이 0이면 'new' 함수는 패닉을 발생한다.
    pub fn new(size: usize) -> ThreadPool {
	assert!(size > 0);

	let mut threads = Vec::with_capacity(size);

	for _ in 0..size {
	    // 스레드를 생성해서 벡터에 저장한다.
	}
	
	ThreadPool {
	    threads
	}
    }

    pub fn execute<F>(&self, f: F)
    where
	F: FnOnce() + Send + 'static
    {
    }
}
