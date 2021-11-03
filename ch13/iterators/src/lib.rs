// 예제 13-23 Counter 반복자에 Iterator 트레이트가 지원하는 다른 메서드 활용

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
	Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
	self.count += 1;

	if self.count < 6 {
	    Some(self.count)
	} else {
	    None
	}
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
	.map(|(a, b)| a * b)
	.filter(|x| x % 3 == 0)
	.sum();

    assert_eq!(18, sum);
}
