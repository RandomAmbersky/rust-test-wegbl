use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::window;
use web_sys::WebGlRenderingContext as GL;

use crate::logger::log;

pub fn get_webgl_context () -> Result<GL, JsValue> {
    log("get_webgl_context");

    let window: web_sys::Window = match window() {
        None => return Err(JsValue::from("Window not found.")),
        Some(t) => t
    };

    let document: web_sys::Document = match window.document() {
        None => return Err(JsValue::from("Document not found.")),
        Some(t) => t
    };

    let element: web_sys::Element = match document.get_element_by_id("rustCanvas") {
        None => return Err(JsValue::from("CanvasElement not found.")),
        Some(t) => t
    };

    let canvas: web_sys::HtmlCanvasElement = match element.dyn_into() {
        Err(e) => return Err(JsValue::from(e)),
        Ok(t) => t
    };

    let gl_object: js_sys::Object = match canvas.get_context("webgl") {
        Err(e) => return Err(JsValue::from(e)),
        Ok(t) => match t {
            None => return Err(JsValue::from("webgl context not found.")),
            Some(t) => t
        }
    };

    let gl_context:GL = match gl_object.dyn_into() {
        Err(e) => return Err(JsValue::from(e)),
        Ok(t) => t
    };
    Ok(gl_context)
}
