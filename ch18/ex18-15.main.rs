// 예제 18-15 다양한 값을 저장하는 열거자의 열것값 해체

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);
    
    match msg {
	Message::Quit => {
	    println!("Quit: 해체할 값이 없습니다.")
	},
	Message::Move { x, y } => {
	    println!(
		"Move: x = {}, y = {}",
		x,
		y
	    );
	},
	Message::Write(text) => println!("Write: {}", text),
	Message::ChangeColor(r, g, b) => {
	    println!(
		"ChangeColor: R = {}, G = {}, B = {}",
		r,
		g,
		b
	    );
	}
    }
}
