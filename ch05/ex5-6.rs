// 예제 5-6 user1 인스턴스의 값 일부를 이용해 User 구조체의 새 인스턴스 생성 코드

let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};
