use specta::{ts::ts_inline, Type};

#[derive(Type)]
struct Optional {
    #[specta(optional)]
    a: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    b: Option<String>,
}

#[test]
fn test() {
    #[cfg(not(feature = "serde"))]
    assert_eq!(ts_inline::<Optional>(), "{ a?: number, b: string | null }");

    #[cfg(feature = "serde")]
    assert_eq!(ts_inline::<Optional>(), "{ a?: number, b?: string }");
}
