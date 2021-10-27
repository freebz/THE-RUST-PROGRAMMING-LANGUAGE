// 예제 10-5 아직 컴파일되지 않지만 제네릭 타입 매개변수를 이용해 선언한 largest 함수

fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
	if item > largest {
	    largest = item;
	}
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("가장 큰 숫자: {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("가장 큰 문자: {}", result);
}
