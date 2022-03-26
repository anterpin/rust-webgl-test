extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    type HTMLDocument;
    type Element;

    static document: HTMLDocument;

    #[wasm_bindgen(method)]
    fn createElement(this: &HTMLDocument, tagName: &str) -> Element;

    #[wasm_bindgen(method, getter)]
    fn body(this: &HTMLDocument) -> Element;

    #[wasm_bindgen(method, js_name = appendChild)]
    fn append(this: &Element, item: Element);

    #[wasm_bindgen(method, setter = innerHTML)]
    fn set_inner(this: &Element, html: &str);
}

#[wasm_bindgen]
pub fn greet(s: &str) {
    alert(&format!("This is from wasm {}", s));
}

#[wasm_bindgen]
pub fn create_stuff() {
    let p = document.createElement("p");
    p.set_inner("vaffanculo");

    let div3 = document.createElement("div");
    div3.append(p);

    let div2 = document.createElement("div");
    div2.append(div3);

    let div = document.createElement("div");
    div.append(div2);

    document.body().append(div);
}
