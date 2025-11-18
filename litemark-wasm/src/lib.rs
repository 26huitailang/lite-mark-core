use js_sys::Uint8Array;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use image::ImageFormat;
use litemark_core::{exif, image_io, layout::Template, renderer::WatermarkRenderer};

#[wasm_bindgen]
pub fn process_image(
    image_bytes: Uint8Array,
    template_json: &str,
    author: Option<String>,
    font_bytes: Option<Uint8Array>,
    logo_bytes: Option<Uint8Array>,
) -> Result<Uint8Array, JsValue> {
    // Copy input bytes from JS
    let mut img_vec = vec![0u8; image_bytes.length() as usize];
    image_bytes.copy_to(&mut img_vec);

    // Decode image using Core
    let mut image = image_io::decode_image(&img_vec)
        .map_err(|e| JsValue::from_str(&format!("decode_image error: {}", e)))?;

    // Extract EXIF from bytes using Core
    let exif_data = exif::extract_from_bytes(&img_vec)
        .map_err(|e| JsValue::from_str(&format!("extract_exif error: {}", e)))?;

    // Prepare variables
    let mut variables: HashMap<String, String> = exif_data.to_variables();
    if let Some(author_name) = author.as_ref() {
        variables.insert("Author".to_string(), author_name.to_string());
    }

    // Parse template JSON
    let template: Template = Template::from_json(template_json)
        .map_err(|e| JsValue::from_str(&format!("template parse error: {}", e)))?;

    // Optional font bytes
    let font_opt = match font_bytes {
        Some(buf) => {
            let mut font_vec = vec![0u8; buf.length() as usize];
            buf.copy_to(&mut font_vec);
            Some(font_vec)
        }
        None => None,
    };

    // Optional logo bytes
    let logo_opt = match logo_bytes {
        Some(buf) => {
            let mut logo_vec = vec![0u8; buf.length() as usize];
            buf.copy_to(&mut logo_vec);
            Some(logo_vec)
        }
        None => None,
    };

    // Create renderer
    let renderer = WatermarkRenderer::from_font_bytes(font_opt.as_deref())
        .map_err(|e| JsValue::from_str(&format!("renderer init error: {}", e)))?;

    // Render watermark
    renderer
        .render_watermark_with_logo_bytes(&mut image, &template, &variables, logo_opt.as_deref())
        .map_err(|e| JsValue::from_str(&format!("render error: {}", e)))?;

    // Encode JPEG output
    let output_bytes = image_io::encode_image(&image, ImageFormat::Jpeg)
        .map_err(|e| JsValue::from_str(&format!("encode error: {}", e)))?;

    // Return as Uint8Array
    let out = Uint8Array::new_with_length(output_bytes.len() as u32);
    out.copy_from(&output_bytes);
    Ok(out)
}

#[wasm_bindgen]
pub fn process_image_basic(
    image_bytes: Uint8Array,
    template_json: &str,
) -> Result<Uint8Array, JsValue> {
    process_image(image_bytes, template_json, None, None, None)
}

#[wasm_bindgen]
pub fn process_batch(
    images: js_sys::Array,
    template_json: &str,
    author: Option<String>,
    font_bytes: Option<Uint8Array>,
    logo_bytes: Option<Uint8Array>,
    on_progress: Option<js_sys::Function>,
) -> Result<js_sys::Array, JsValue> {
    use wasm_bindgen::JsCast;

    let total = images.length();

    // Parse template once
    let template: Template = Template::from_json(template_json)
        .map_err(|e| JsValue::from_str(&format!("template parse error: {}", e)))?;

    // Prepare renderer once
    let font_opt = match font_bytes {
        Some(buf) => {
            let mut font_vec = vec![0u8; buf.length() as usize];
            buf.copy_to(&mut font_vec);
            Some(font_vec)
        }
        None => None,
    };

    let renderer = WatermarkRenderer::from_font_bytes(font_opt.as_deref())
        .map_err(|e| JsValue::from_str(&format!("renderer init error: {}", e)))?;

    // Prepare logo bytes once
    let logo_opt = match logo_bytes {
        Some(buf) => {
            let mut logo_vec = vec![0u8; buf.length() as usize];
            buf.copy_to(&mut logo_vec);
            Some(logo_vec)
        }
        None => None,
    };

    let outputs = js_sys::Array::new();

    for i in 0..total {
        let val = images.get(i);
        let u8 = val
            .dyn_into::<Uint8Array>()
            .map_err(|_| JsValue::from_str("Expected Uint8Array"))?;

        // Copy bytes
        let mut img_vec = vec![0u8; u8.length() as usize];
        u8.copy_to(&mut img_vec);

        // Decode
        let mut image = image_io::decode_image(&img_vec)
            .map_err(|e| JsValue::from_str(&format!("decode_image error: {}", e)))?;

        // EXIF
        let exif_data = exif::extract_from_bytes(&img_vec)
            .map_err(|e| JsValue::from_str(&format!("extract_exif error: {}", e)))?;

        // Variables
        let mut vars: HashMap<String, String> = exif_data.to_variables();
        if let Some(author_name) = author.as_ref() {
            vars.insert("Author".to_string(), author_name.to_string());
        }

        // Render
        renderer
            .render_watermark_with_logo_bytes(&mut image, &template, &vars, logo_opt.as_deref())
            .map_err(|e| JsValue::from_str(&format!("render error: {}", e)))?;

        // Encode
        let output_bytes = image_io::encode_image(&image, ImageFormat::Jpeg)
            .map_err(|e| JsValue::from_str(&format!("encode error: {}", e)))?;
        let out = Uint8Array::new_with_length(output_bytes.len() as u32);
        out.copy_from(&output_bytes);
        outputs.push(&out);

        // Progress callback
        if let Some(cb) = on_progress.as_ref() {
            let this = JsValue::NULL;
            let completed = JsValue::from_f64((i + 1) as f64);
            let total_js = JsValue::from_f64(total as f64);
            let _ = cb.call2(&this, &completed, &total_js);
        }
    }

    Ok(outputs)
}
