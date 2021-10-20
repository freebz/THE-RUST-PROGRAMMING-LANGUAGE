// 예제 8-16 String 타입에 변수의 데이터를 덧붙인 후 해당 변수를 다시 사용

let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2: {}", s2);
