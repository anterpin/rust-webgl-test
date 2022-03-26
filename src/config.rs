use wasm_bindgen::prelude::*;

pub static mut VALUE: i32 = 2;

#[wasm_bindgen]
pub fn set_value(v: i32) {
    unsafe {
        VALUE = v;
    }
}
