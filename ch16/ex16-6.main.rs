// 예제 16-6 채널을 생성하고 전달자(tx)와 수신자(rx) 대입

use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}
