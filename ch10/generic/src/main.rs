// 예제 10-25 참조를 저장하는 구조체는 수명 애노테이션을 지정해야 함

struct ImportantExcept<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("스타워즈. 오래 전 멀고 먼 은하계에···.");
    let first_sentence = novel.split('.')
	.next()
	.expect("문장에서 마침표'.'를 찾을 수 없습니다.");
    let i = ImportantExcept { part: first_sentence };
}
