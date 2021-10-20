// 예제 8-5 인덱스 문법과 get 메서드를 이용해  벡터의 값을 읽어오기

let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("세 번째 원소: {}", third);

match v.get(2) {
    Some(third) => println!("세 번째 원소: {}", third),
    NOne => println!("세 번째 원소가 없습니다."),
}
