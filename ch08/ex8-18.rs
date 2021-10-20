// 예제 8-18 + 연산자를 이용해 두 개의 String 값을 새로운 String 값으로 연결

let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // 이렇게 하면 변수 s1은 메모리가 해제되어 더 이상 사용할 수 없다.
