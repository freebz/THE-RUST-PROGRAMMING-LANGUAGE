// 예제 5-9 튜플을 이용해 사각형의 너비와 높이 표현

fn main() {
    let rect1 = (30, 50);
    
    println!(
	"사각형의 면적: {} 제곱 픽셀",
	area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
