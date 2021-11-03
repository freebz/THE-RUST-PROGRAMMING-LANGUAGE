// 예제 13-9 클로저와 리턴값을 calculation과 value 필드에 저장하는 Cacher 구조체

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}
