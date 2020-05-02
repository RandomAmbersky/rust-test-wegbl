extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

#[macro_use]
extern crate lazy_static;

mod app_state;
mod gl_setup;
mod programs;
mod common_funcs;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn say_hello_from_rust() {
    log("Howdy!... from Rust =)")
}

#[wasm_bindgen]
pub struct AmberSkyNet {
    gl: WebGlRenderingContext,
    program_color_2d: programs::Color2D
}

#[wasm_bindgen]
impl AmberSkyNet {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        log("AmberSkyNet new");
        let gl = gl_setup::initialize_webgl_context().unwrap();
        let program_color_2d = programs::Color2D::new(&gl);
        Self {
            gl,
            program_color_2d
        }
    }

    pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue>{
        app_state::update_dynamic_data(_time, _height, _width);
        let log_msg = _time.to_string();
        log(&log_msg);
        Ok(())
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT );
        let curr_state = app_state::get_curr_state();

        self.program_color_2d.render(
            &self.gl,
            curr_state.control_bottom,
            curr_state.control_top,
            curr_state.control_left,
            curr_state.control_right,
            curr_state.canvas_height,
            curr_state.canvas_width,
        );
    }
}
