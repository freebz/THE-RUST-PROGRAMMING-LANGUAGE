// 예제 8-6 다섯 개의 값을 가진 벡터의 100번째 인덱스값에 접근

let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100];
let does_not_exist = v.get(100);
