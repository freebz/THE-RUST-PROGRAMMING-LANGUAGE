// 예제 13-28 예제 12-19의 search 함수

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines() {
	if line.contains(query) {
	    results.push(line);
	}
    }

    results
}
