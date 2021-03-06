// 예제 20-24 Message 값을 전달하고 Worker 인스턴스가 Message::Terminate 값을 수신하면 루프 종료

use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
	(*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

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

	self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
	for worker in &mut self.workers {
	    println!("종료: 작업자 {}", worker.id);

	    if let Some(thread) = worker.thread.take() {
		thread.join().unwrap();
	    }
	}
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
	let thread = thread::spawn(move || {
	    loop {
		let message = receiver.lock().unwrap().recv().unwrap();

		match message {
		    Message::NewJob(job) => {
			println!("시작: 작업자 {}", id);

			job.call_box();
		    },
		    Message::Terminate => {
			println!("종료 메시지 수신: 작업자 {}", id);
			break;
		    }
		}
	    }
	});

	Worker {
	    id,
	    thread: Some(thread),
	}
    }
}
