// 예제 15-20 현재 값이 최댓값에 얼마나 가까워지는지 추적해서 어느 수준이 되면 경고를 내보내는 라이브러리

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
	LimitTracker {
	    messenger,
	    value: 0,
	    max,
	}
    }

    pub fn set_value(&mut self, value: usize) {
	self.value = value;

	let percentage_of_max = self.value as f64 / self.max as f64;

	if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
	    self.messenger.send("경고: 최댓값의 75%를 사용했습니다.");
	} else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
	    self.messenger.send("긴급 경고: 최댓값의 90%를 사용했습니다.");
	} else if percentage_of_max >= 1.0 {
	    self.messenger.send("에러: 최댓값을 초과했습니다.");
	}
    }
}
