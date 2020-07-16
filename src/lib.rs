mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    using_web_sys();
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}", name));
    log("console hello");
}

pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn using_web_sys() {
    use web_sys::console;

    console::log_1(&"Hello using web-sys".into());
    let result = add(4, 8);

    let js: JsValue = 4.into();
    // let js: JsValue = 4.into();
    console::log_2(&"Logging arbitrary values looks like".into(), &result.into());
    // console::log_2(&"Logging arbitrary values looks like".into(), &js);
}

#[wasm_bindgen]
struct Rectangle {
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl Rectangle {
    pub fn new(width:u32, height: u32) -> Rectangle {
        Rectangle {width, height}
    }

    pub fn new_default() -> Rectangle {
        Rectangle {width: 100, height: 100}
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

