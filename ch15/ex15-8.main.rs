// 예제 15-8 MyBox<T> 타입 선언

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
	MyBox(x)
    }
}
