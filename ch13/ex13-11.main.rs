// 예제 13-11 generate_workout 함수에서 Cacher 구조체를 이용해 캐싱 로직 추상화

use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
	simulated_user_specified_value,
	simulated_random_number
    );
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
	println!("시간이 오래 걸리는 계산을 수행 중···");
	thread::sleep(Duration::from_secs(2));
	num
    });

    if intensity < 25 {
	println!(
	    "오늘은 {}번의 팔 굽혀펴기를 하세요!",
	    expensive_result.value(intensity)
	);
	println!(
	    "다음에는 {}번의 윗몸 일으키기를 하세요!",
	    expensive_result.value(intensity)
	);
    } else {
	if random_number == 3 {
	    println!("오늘은 수분을 충분히 섭취하며 쉬세요!");
	} else {
	    println!(
		"오늘은 {}분간 달리기를 하세요!",
		expensive_result.value(intensity)
	    );
	}
    }
}

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
	Cacher {
	    calculation,
	    value: None,
	}
    }

    fn value(&mut self, arg: u32) -> u32 {
	match self.value {
	    Some(v) => v,
	    None => {
		let v = (self.calculation)(arg);
		self.value = Some(v);
		v
	    }
	}
    }
}
