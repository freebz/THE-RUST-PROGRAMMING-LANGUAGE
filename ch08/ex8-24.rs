// 예제 8-24 키에 저장된 값 교체

use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("블루"), 10);
scores.insert(String::from("블루"), 25);

println!("{:?}", scores);
