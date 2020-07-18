// use serde::{Serialize, Deserialize};
mod utils;

use wasm_bindgen::prelude::*;

// #[wasm_bindgen(start)]
// pub fn run() {
//     using_web_sys();
// }

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    // fn alert(s: &str);

    // #[wasm_bindgen(js_namespace = console)]
    // fn log(s: &str);
}

// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     alert(&format!("Hello, {}", name));
//     log("console hello");
// }

// fn using_web_sys() {
//     use web_sys::console;

//     console::log_1(&"Hello using web-sys".into());
//     let result = add(4, 8);

//     let js: JsValue = 4.into();
//     // let js: JsValue = 4.into();
//     console::log_2(&"Logging arbitrary values looks like".into(), &result.into());
//     // console::log_2(&"Logging arbitrary values looks like".into(), &js);
// }

#[wasm_bindgen]
pub struct Rectangle {
    width: u32,
    height: u32,
    pos_x: i32,
    pos_y: i32,
}

#[wasm_bindgen]
impl Rectangle {
    pub fn new(width:u32, height: u32, pos_x: i32, pos_y: i32) -> Rectangle {
        Rectangle {width, height, pos_x, pos_y}
    }

    pub fn new_default() -> Rectangle {
        Rectangle {width: 100, height: 100, pos_x: 0, pos_y: 0}
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    // pub fn move_right(&mut self, x: u32) {
    pub fn move_right(&mut self) {
        self.width = self.width + 10;
    }
}


#[wasm_bindgen]
pub struct Game {
    cells: Vec<Rectangle>,
}

#[wasm_bindgen]
impl Game {

    pub fn new() -> Game {
        Game {cells: vec![]}
    }

    pub fn paint_grid(&mut self, colums: u32) {
        // let w = 65000;
        let w = 4294967295;
        let h = 20;
        let mut pos_x: i32 = 40;
        let pos_y = 100;
        for i in 0..colums {
            pos_x = 40 + (i as i32 * 20);
            let rect = Rectangle::new(w, h, pos_x, pos_y);
            self.cells.push(rect);
        }
    }

    pub fn cells(&self) -> *const Rectangle {
        self.cells.as_ptr()
    }

    pub fn cells_len(&self) -> usize {
        self.cells.len()
    }

    pub fn get_rect_pos_x(&self, index: usize) -> i32 {
        self.cells[index].pos_x
    }

    
}