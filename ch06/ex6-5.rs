// 예제 6-5 match 표현식을 이용한 Option<i32> 타입 확인 함수

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
	None => None,
	Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
