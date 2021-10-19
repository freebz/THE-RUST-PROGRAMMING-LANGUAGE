// 예제 5-5 변수와 필드 이름이 일치하므로 필드 초기화 단축 문법을 사용해 다시 작성한 build_user 함수

fn build_user(email: String, username: String) -> User {
    User {
	email,
	username,
	active: true,
	sign_in_count: 1,
    }
}
