// 예제 18-19 튜플의 여러 부분을 무시하는 코드

fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
	(first, _, third, _, fifth) => {
	    println!("일치하는 숫자: {}, {}, {}", first, third, fifth)
	},
    }
}
