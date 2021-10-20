// 예제 8-23 해시 맵에서 블루팀의 점수 읽기

use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("블루"), 10);
scores.insert(String::from("옐로"), 50);

let team_name = String::from("블루");
let score = scores.get(&team_name);
