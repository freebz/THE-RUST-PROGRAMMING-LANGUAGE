// 예제 13-1 실행에 2초가 걸리는 가상의 계산을 수행하는 함수

use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("시간이 오래 걸리는 계산을 수행 중···");
    thread::sleep(Duration::from_secs(2));
    intensity
}
