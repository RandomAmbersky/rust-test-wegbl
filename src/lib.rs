extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

mod logger;
mod render;

use logger::log as log;

#[wasm_bindgen]
pub fn say_hello_from_rust() {
    log("Howdy!... from Rust =)")
}

#[wasm_bindgen]
pub struct AmberSkyNet {
    render_webgl: render::Render
}

#[wasm_bindgen]
impl AmberSkyNet {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        log("AmberSkyNet new()");

        let render = render::Render::new();
        Self {
            render_webgl: render
        }
    }

    pub fn update(&mut self, _time: f32, _height: f32, _width: f32) {
        // log("AmberSkyNet update");
    }

    pub fn render(&self) {
        self.render_webgl.draw();
        // log("AmberSkyNet render");
    }
}
