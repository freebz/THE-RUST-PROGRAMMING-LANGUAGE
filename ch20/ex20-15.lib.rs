// 예제 20-15 스레드 말고 Worker 인스턴스를 저장하는 ThreadPool 구조체

use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,    
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

	let mut workers = Vec::with_capacity(size);

	for id in 0..size {
	    workers.push(Worker::new(id))
	}
	
	ThreadPool {
	    workers
	}
    }

    pub fn execute<F>(&self, f: F)
    where
	F: FnOnce() + Send + 'static
    {
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
	let thread = thread::spawn(|| {});

	Worker {
	    id,
	    thread,
	}
    }
}
