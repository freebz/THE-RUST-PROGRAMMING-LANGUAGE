// 예제 19-22 Display 트레이트의 기능에 의존하는 OutlinePrint 트레이트 구현

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
	let output = self.to_string();
	let len = output.len();
	println!("{}", "*".repeat(len + 4));
	println!("*{}*", " ".repeat(len + 2));
	println!("* {} * ", output);
	println!("*{}*", " ".repeat(len + 2));
	println!("{}", "*".repeat(len + 4));
    }
}
