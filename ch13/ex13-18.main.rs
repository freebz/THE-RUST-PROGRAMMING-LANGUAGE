// 예제 13-18 map 메서드로 새로운 반복자 생성 후 collect 메서드로 벡터 생성

fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}
