// 예제 5-4 메일 주소와 사용자 이름을 전달받아 새로운 User 인스턴스를 리턴하는 build_user 함수

fn build_user(email: String, username: String) -> User {
    User {
	email: email,
	username: username,
	active: true,
	sign_in_count: 1,
    }
}
