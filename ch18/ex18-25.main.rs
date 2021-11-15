// 예제 18-25 ..를 모호하게 사용하는 예

fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
	(.., second, ..) => {
	    println!("second = {}", second);
	}
    }
} 
