use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

use crate::logger::log;

pub fn get_webgl_context () -> Result<GL, JsValue> {
    log("get_webgl_context");
    let window: web_sys::Window = window().unwrap();
    let document: web_sys::Document = window.document().unwrap();
    let element: web_sys::Element = document.get_element_by_id("rustCanvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = element.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    let gl:WebGlRenderingContext = canvas.get_context("webgl")?.unwrap().dyn_into()?;
    Ok(gl)
}
