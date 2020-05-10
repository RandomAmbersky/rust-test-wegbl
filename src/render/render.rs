use web_sys::WebGlRenderingContext as GL;

use crate::render::gl_setup::get_webgl_context;
// use crate::logger::log;
use wasm_bindgen::JsValue;

pub struct Render {
    gl: GL
}

impl Render {
    pub fn new () -> Self {
        let gl: GL = get_webgl_context().unwrap();
        gl.enable(GL::BLEND);
        gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
        gl.clear_color(1.0, 1.0, 0.0, 1.0); //RGBA
        gl.clear_depth(1.0);
        Self {
            gl
        }
    }
    pub fn draw (&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT );
    }
}
