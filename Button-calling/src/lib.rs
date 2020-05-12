mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlButtonElement};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn start() {
    utils::set_panic_hook();
    let callback = Closure::wrap(Box::new(add_message) as Box<dyn Fn()>);

    // Create a button and append to DOM
    let document = window().unwrap().document().unwrap();
    let button = document
        .create_element("button")
        .unwrap()
        .dyn_into::<HtmlButtonElement>()
        .unwrap();
    button.set_inner_html("CallBack function Button");
    document.body().unwrap().append_child(&button).unwrap();

    button.set_onclick(Some(callback.as_ref().unchecked_ref()));

    callback.forget();
}

fn add_message() {
    let document = window().unwrap().document().unwrap();
    let element = document.create_element("p").unwrap();
    element.set_inner_html("Hello from Rust callback Function!");
    document.body().unwrap().append_child(&element).unwrap();
}
