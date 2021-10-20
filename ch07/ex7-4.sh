# 예제 7-4 예제 7-3의 코드를 컴파일하면 출력되는 에러 메시지

cargo build
# Compiling restaurant v0.1.0 (file:///projects/restaurant)
# error[E0603]: module `hosting` is private
#   --> src/lib.rs:9:28
#    |
# 9  |     crate::front_of_house::hosting::add_to_waitlist();
#    |                            ^^^^^^^ private module
#    |
# error[E0603]: module `hosting` is private
#   --> src/lib.rs:12:21
#    |
# 12 |     front_of_house::hosting::add_to_waitlist();
#    |                     ^^^^^^^ private module
#    |
