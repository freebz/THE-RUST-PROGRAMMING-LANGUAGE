// 예제 13-2 사용자의 입력과 임의의 숫자를 하드코딩한 값을 이용하는 main 함수

use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("시간이 오래 걸리는 계산을 수행 중···");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
	simulated_user_specified_value,
	simulated_random_number
    );
}
