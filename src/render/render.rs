use std::rc::Rc;
use web_sys::WebGlRenderingContext as GL;
use crate::render::gl_setup;
use crate::render::gl_texture::Texture;

pub struct Render {
    gl: Rc<GL>,
    tex: Texture
}

impl Render {
    pub fn new () -> Self {

        let gl_ctx: GL = gl_setup::get_webgl_context().unwrap();
        let gl: Rc<GL> = Rc::new(gl_ctx);
        let tex: Texture = Texture::new(gl.clone());

        gl.enable(GL::BLEND);
        gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
        gl.clear_color(1.0, 1.0, 0.0, 1.0); //RGBA
        gl.clear_depth(1.0);

        Self {
            gl,
            tex
        }
    }
    pub fn draw (&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT );
    }
}
