// 예제 13-17 새로운 반복자를 생성하는 반복자 어댑터 map 메서드

fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    v1.iter().map(|x| x + 1);
}
