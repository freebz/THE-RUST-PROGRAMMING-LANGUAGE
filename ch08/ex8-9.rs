// 예제 8-9 벡터에 저장된 값에 대한 가변 참조 순회

let mut v = vec![1, 2, 3, 4, 5];
for i in &mut v {
    *i += 50;
}
