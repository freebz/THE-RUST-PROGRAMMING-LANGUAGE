// 예제 8-20 새로운 해시 맵을 생성하고 키와 값 추가

use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("블루"), 10);
scores.insert(String::from("옐로"), 50);
