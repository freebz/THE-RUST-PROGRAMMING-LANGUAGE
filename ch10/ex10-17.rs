// 예제 10-17 범위를 벗어난 값 참조 시도

{
    let r;
    {
	let x = t;
	r = &x;
    }

    println!("r: {}", r);
}
