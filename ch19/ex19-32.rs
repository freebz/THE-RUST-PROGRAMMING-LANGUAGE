// 예제 19-32 예제 19-30에서 매크로 특성이 적용된 코드를 구문분석한 후 얻게된 DeriveInput 인스턴스의 구조

DeriveInput {
    // 생략
    ident: Ident {
	ident: "Pancakes",
	span: #0 bytes(95..103)
    },
    data: Struct(
	DataStruct {
	    struct_token: Struct,
	    fields: Unit,
	    semi_token: Some(
		Semi
	    )
	}
    }
}
