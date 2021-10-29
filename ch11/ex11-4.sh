# 예제 11-4 하나의 테스트는 성공하고 다른 하나는 실패한 경우의 테스트 결과

cargo test
#    Compiling adder v0.1.0 (/rust/projects/adder)
#     Finished test [unoptimized + debuginfo] target(s) in 1.44s
#      Running target/debug/deps/adder-6f6d09e2972de52b

# running 2 tests
# test tests::exploration ... ok
# test tests::another ... FAILED

# failures:

# ---- tests::another stdout ----
# thread 'tests::another' panicked at '테스트 실패!', src/lib.rs:12:5
# note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


# failures:
#     tests::another

# test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

# error: test failed, to rerun pass '--lib'
