pub mod simple_lists {
    use ::tauri_bindgen_guest_rust::bitflags;
    use ::tauri_bindgen_guest_rust::tauri_bindgen_abi;
    pub async fn simple_list1(l: &'_ [u32]) -> () {
        todo!()
    }
    pub async fn simple_list2() -> Vec<u32> {
        todo!()
    }
    pub async fn simple_list3(a: &'_ [u32], b: &'_ [u32]) -> (Vec<u32>, Vec<u32>) {
        todo!()
    }
    pub async fn simple_list4(l: &'_ [&'_ [u32]]) -> Vec<Vec<u32>> {
        todo!()
    }
}
