// 예제 10-1 숫자의 리스트로부터 가장 큰 숫자 찾기

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
	if number > largest {
	    largest = number;
	}
    }

    println!("가장 큰 숫자: {}", largest);
}
