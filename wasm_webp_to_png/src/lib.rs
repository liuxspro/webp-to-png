use image::{ImageError, ImageFormat};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

fn handle_error(err: ImageError) -> JsValue {
    JsValue::from_str(&format!("Error haha: {:?}", err))
}

#[wasm_bindgen]
pub fn webp_to_png(webp_data: Vec<u8>) -> Result<Vec<u8>, JsValue> {
    // 使用 Cursor 将 Vec<u8> 转换为可读的流
    let reader = Cursor::new(webp_data);
    // 解码 WebP 图片
    let img = image::load(reader, ImageFormat::WebP).map_err(handle_error)?;
    // 创建一个 Vec<u8> 来存储 PNG 数据
    let mut png_data = Vec::new();
    // 将图片保存为 PNG 格式
    img.write_to(&mut Cursor::new(&mut png_data), ImageFormat::Png)
        .map_err(handle_error)?;

    Ok(png_data)
}
