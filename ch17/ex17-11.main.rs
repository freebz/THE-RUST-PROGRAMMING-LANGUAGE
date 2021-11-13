// 예제 17-11 blog 크레이트에 구현한 동작을 보여주는 코드

use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("나는 오늘 점심으로 샐러드를 먹었다.");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("나는 오늘 점심으로 샐러드를 먹었다.", post.content());
}
