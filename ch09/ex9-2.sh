# 예제 9-2 RUSH_BACKTRACE 환경 변수를 설정한 후 panic! 매크로 호출에 의해 생성된 역추적 정보

RUST_BACKTRACE=1 cargo run
#     Finished dev [unoptimized + debuginfo] target(s) in 0.06s
#      Running `target/debug/panic`
# thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:6:5
# stack backtrace:
#    0: rust_begin_unwind
#              at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:483
# -- 생략 --
#    7: core::ops::function::FnOnce::call_once
#              at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/ops/function.rs:227
# -- 생략 --
