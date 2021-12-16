// 예제 19-28 vec! 매크로 정의 간소화

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
	{
	    let mut temp_vec = Vec::new();
	    $(
		temp_vec.push($x);
	    )*
	    temp_vec
	}
    };
}
