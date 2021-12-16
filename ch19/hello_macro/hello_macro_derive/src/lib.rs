// 예제 19-33 구문분석한 러스트 코드로 HelloMacro 트레이트를 구현하는 코드

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

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
	impl HelloMacro for #name {
	    fn hello_macro() {
		println!("안녕하세요 매크로! 내 이름은 {}입니다!", stringify(#name));
	    }
	}
    }
    gen.into()
}
