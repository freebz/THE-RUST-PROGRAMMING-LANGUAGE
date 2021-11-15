// 예제 18-7 튜플을 해체하는 매개변수를 가진 함수

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("현재 위치: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
