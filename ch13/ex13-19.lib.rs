// 예제 13-19 shoe_size 변수를 캡처한 클로저를 filter 메서드에 전달

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
	.filter(|s| s.size == shoe_size)
	.collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
	Shoe { size: 10, style: String::from("스니커즈") },
	Shoe { size: 13, style: String::from("샌달") },
	Shoe { size: 10, style: String::from("부츠") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
	in_my_size,
	vec![
	    Shoe { size: 10, style: String::from("스니커즈") },
	    Shoe { size: 10, style: String::from("부츠") },
	]
    );
}
