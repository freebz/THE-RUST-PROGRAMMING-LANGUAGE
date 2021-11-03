// 예제 13-12 자신 주변에 선언된 변수를 참조하는 클로저

fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    //println!("변수 x를 사용할 수 없습니다: {:?}", x);
    
    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
