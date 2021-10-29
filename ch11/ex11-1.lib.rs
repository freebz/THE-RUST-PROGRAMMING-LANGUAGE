// 예제 11-1 cargo new 명령이 자동으로 생성한 테스트 모듈과 함수

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
