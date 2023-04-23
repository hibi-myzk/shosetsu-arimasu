use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let text = body.inner_html().replace("知らんけど", "諸説あります");
    body.set_inner_html(&text);
}