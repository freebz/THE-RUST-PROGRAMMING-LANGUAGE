// 예제 8-21 팀의 리스트와 팀의 점수를 이용해 해시 맵 생성

use std::collections::HashMap;

let teams = vec![String::from("블루"), String::from("옐로")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
