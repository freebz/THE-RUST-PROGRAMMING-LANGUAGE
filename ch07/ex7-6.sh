# 예제 7-6 예제 7-5를 컴파일하면 출력되는 에러 메시지

cargo build
# Compiling restaurant v0.1.0 (file:///restaurant)
# error[E0603]: function `add_to_waitlist` is private
#   --> src/lib.rs:9:37
#    |
# 9  |     crate::front_of_house::hosting::add_to_waitlist();
#    |                                     ^^^^^^^^^^^^^^^ private function
#    |
# error[E0603]: function `add_to_waitlist` is private
#   --> src/lib.rs:12:30
#    |
# 12 |     front_of_house::hosting::add_to_waitlist();
#    |                              ^^^^^^^^^^^^^^^ private function
#    |
