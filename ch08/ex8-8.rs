// 예제 8-8 for 루프를 이용해 벡터를 순회하면서 저장된 값 출력

let v = vec![1, 2, 3, 4, 5];
for i in &v {
    println!("{}", i);
}
