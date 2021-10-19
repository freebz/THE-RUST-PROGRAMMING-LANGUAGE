// 예제 5-7 구조체 갱신 문법을 이용해 email과 username 필드에는 새로운 값을 대입하고 나머지 필드에는 user1 변수에 저장된 인스턴스의 필드값들을 대입하는 새 인스턴스 생성 코드

let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
