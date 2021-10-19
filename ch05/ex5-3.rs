// 예제 5-3 User 인스턴스의 email 필드값 변경

let mut user1 = User {
    email: String::from("someone@exmple.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
