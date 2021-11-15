// 예제 18-18 패턴 안에서 밑줄을 사용해서 Some 열것값을 확인하지만 그 안에 저장된 실제 값은 사용하지 않는 코드

fn main() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
	(Some(_), Some(_)) => {
	    println!("이미 설정된 값을 덮어쓸 수 없습니다.");
	}
	_ => {
	    setting_value = new_setting_value;
	}
    }

    println!("현재 설정: {:?}", setting_value);
}
