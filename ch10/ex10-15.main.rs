// 예제 10-15 PartialOrd와 Copy 트레이트를 구현하는 제네릭 타입에 대해서만 동작하는 완성된 largest 함수

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
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
