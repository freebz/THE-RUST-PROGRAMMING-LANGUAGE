// 예제 8-22 키와 값을 해시 맵에 추가한 후 소유권 이동

use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("블루");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name과 field_value 변수는 이 지점부터 유효하지 않다.
// 이 값들을 사용하려고 하면 컴파일러가 에러를 발생한다.
