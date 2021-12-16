// 예제 19-26 continue로 끝나는 가지를 가진 match 표현식

let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
