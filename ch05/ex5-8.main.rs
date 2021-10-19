// 예제 5-8 width와 height 변수를 이용해 사각형 면적 구하기

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
	"사각형의 면적: {} 제곱 픽셀",
	area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
