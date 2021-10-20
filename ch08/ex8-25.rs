// 예제 8-25 entry 메서드를 이용해서 키에 값이 할당되어 있지 않을 때만 값 추가

use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("블루"), 10);

scores.entry(String::from("옐로")).or_insert(50);
scores.entry(String::from("블루")).or_insert(50);

println!("{:?}", scores);
