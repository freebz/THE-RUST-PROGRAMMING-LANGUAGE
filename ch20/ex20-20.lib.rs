// 예제 20-20 Worker 스레드에서 작업 수신 및 실행

use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<FnOnce() + Send + 'static>;

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

	let (sender, receiver) = mpsc::channel();

	let receiver = Arc::new(Mutex::new(receiver));

	let mut workers = Vec::with_capacity(size);

	for id in 0..size {
	    workers.push(Worker::new(id, Arc::clone(&receiver)));
	}
	
	ThreadPool {
	    workers,
	    sender,
	}
    }

    pub fn execute<F>(&self, f: F)
    where
	F: FnOnce() + Send + 'static
    {
	let job = Box::new(f);

	self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
	let thread = thread::spawn(move || {
	    loop {
		let job = receiver.lock().unwrap().recv().unwrap();

		println!("시작: 작업자 {}", id);

		(*job)();
	    }
	});

	Worker {
	    id,
	    thread,
	}
    }
}
