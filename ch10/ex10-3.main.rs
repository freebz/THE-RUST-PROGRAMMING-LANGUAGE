// 예제 10-3 두 개의 리스트에서 가장 큰 숫자 찾기 추상화

fn largest(list: &[i32]) -> i32 {
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

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("가장 큰 숫자: {}", result);
}
