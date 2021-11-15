// 예제 18-1 if let, else if, else if let과 else를 혼합

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
	println!("선호하는 {}색을 배경으로 사용합니다.", color);
    } else if is_tuesday {
	println!("화요일엔 녹색이죠!");
    } else if let Ok(age) = age {
	if age > 30 {
	    println!("보라색을 배경으로 사용합니다.");
	} else {
	    println!("오렌지색을 배경으로 사용합니다.");
	}
    } else {
	println!("파란색을 배경으로 사용합니다.");
    }
}
