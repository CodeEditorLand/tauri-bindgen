#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod chars {
    use ::tauri_bindgen_guest_rust::serde;
    use ::tauri_bindgen_guest_rust::bitflags;
    pub type A<'a> = &'a str;
    ///A function that accepts a character
    pub async fn take_char(x: char) {
        ::tauri_bindgen_guest_rust::invoke("chars", "take_char", &(x)).await.unwrap()
    }
    ///A function that returns a character
    pub async fn return_char() -> A {
        ::tauri_bindgen_guest_rust::invoke("chars", "return_char", &()).await.unwrap()
    }
}
