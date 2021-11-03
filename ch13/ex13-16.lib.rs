// 예제 13-16 반복자 내 아이템 총합을 얻기 위해 sum 메서드 호출

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
