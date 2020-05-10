use std::rc::Rc;
use wasm_bindgen::prelude::*;

use web_sys::WebGlRenderingContext as GL;
use web_sys::HtmlImageElement;

use crate::logger::log;

pub struct Texture {
    width: i32,
    height: i32,
    state: bool
}

impl Texture {
    pub fn new (gl: Rc<GL>) -> Self {
        log("Texture new()");
        load_image(gl.clone(), "tiles.png");
        Self {
            width: 0,
            height: 0,
            state: false
        }
    }
}

fn load_image (gl: Rc<GL>, src: &str) {
    let mess = format!("Try load image '{}'", src);
    log(&mess);

    let image = HtmlImageElement::new().unwrap();
    let texture = gl.create_texture();

    // let promise = js_sys::Promise::new(&mut |resolve, reject| {
    //     image.set_src(src);
    //     image.set_onload(Some(&resolve));
    //     image.set_onerror(Some(&reject));
    // });
    //
    // wasm_bindgen_futures::JsFuture::from(promise);

}
