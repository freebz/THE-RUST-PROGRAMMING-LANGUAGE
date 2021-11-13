// 예제 17-21 새로운 블로그 포스트 업무 흐름에 맞춰 수정한 main 함수

use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("나는 오늘 점심으로 샐러드를 먹었다.");

    let post = post.request_review();
    
    let post = post.approve();
    assert_eq!("나는 오늘 점심으로 샐러드를 먹었다.", post.content());
}
