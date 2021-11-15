// 예제 18-24 튜플의 처음과 마지막 값만 비교하며 나머지는 무시하기

fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
	(first, .., last) => {
	    println!("first = {}, last = {}", first, last);
	}
    }
} 
