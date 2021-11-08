// 예제 15-2 i32 값의 콘스 리스트 데이터 구조를 표현하는 열거자 선언

enum List {
    Cons(i32, List),
    Nil
}
