// 예제 19-31 러스트 코드를 처리하는 대부분 절차적 매크로 크레이트 구성 코드

extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 전달된 러스트 코드를 조작할 수 있도록 문법 트리를 구성한다.
    let ast = syn::parse(input).unwrap();
    // 트레이트 구현체를 빌드한다.
    impl_hello_macro(&ast)
}
