use js_sys::Function;
use wasm_bindgen::prelude::*;

mod utils;

#[wasm_bindgen]
pub fn md5(bytes: &[u8], batch_size: usize, callback: Option<Function>) -> Result<String, String> {
    let mut context = md5::Context::new();
    let total = bytes.len();
    let mut md5_size = 0;
    for chunk in bytes.chunks(batch_size) {
        context.consume(chunk);
        md5_size += chunk.len();
        if let Some(ref cb) = callback {
            let process = (md5_size as f64 / total as f64 * 100.0).min(100.0);
            let x = JsValue::from(process);
            cb.call1(&JsValue::NULL, &x).unwrap();
        }
    }
    let digest = context.compute();
    Ok(format!("{:x}", digest))
}