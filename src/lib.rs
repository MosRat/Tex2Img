use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint};
use std::path::PathBuf;
use anyhow::{Result, anyhow};
use pdfium_render::prelude::*;
use image::ImageFormat;

// 错误处理结构体
#[repr(C)]
pub struct FfiError {
    message: *mut c_char,
    code: c_int,
}

impl FfiError {
    fn new(message: String, code: i32) -> Self {
        let c_string = CString::new(message).unwrap();
        FfiError {
            message: c_string.into_raw(),
            code,
        }
    }
}

// 释放错误内存
#[unsafe(no_mangle)]
pub extern "C" fn free_error(error: *mut FfiError) {
    if !error.is_null() {
        unsafe {
            let ffi_error = Box::from_raw(error);
            let _ = CString::from_raw(ffi_error.message);
        }
    }
}

// 导出函数：将 PDF 数据转换为 PNG 图片
#[unsafe(no_mangle)]
pub extern "C" fn pdf_to_png(
    data: *const u8,
    data_len: c_uint,
    name: *const c_char,
    path: *const c_char,
    width: c_uint,
    error: *mut *mut FfiError,
) -> c_int {
    let result = _pdf_to_png(data, data_len, name, path, width);
    match result {
        Ok(pages) => pages as c_int,
        Err(e) => {
            let ffi_error = Box::new(FfiError::new(e.to_string(), 1));
            unsafe {
                *error = Box::into_raw(ffi_error);
            }
            -1
        }
    }
}

fn _pdf_to_png(
    data: *const u8,
    data_len: c_uint,
    name: *const c_char,
    path: *const c_char,
    width: c_uint,
) -> Result<usize> {
    // 转换 C 字符串到 Rust 字符串
    let name_cstr = unsafe { CStr::from_ptr(name) };
    let name_str = name_cstr.to_str()?;
    
    let path_cstr = unsafe { CStr::from_ptr(path) };
    let path_str = path_cstr.to_str()?;
    let path_buf = PathBuf::from(path_str);
    
    // 转换 PDF 数据
    let data_slice = unsafe { std::slice::from_raw_parts(data, data_len as usize) };
    let data_vec = data_slice.to_vec();
    
    // 调用原始函数
    export_pdf_to_png(data_vec, name_str, &path_buf, width as u32)
}

fn export_pdf_to_png(data: Vec<u8>, name: &str, path: &PathBuf, width: u32) -> Result<usize> {
    let pdfium = Pdfium::default();
    let document = pdfium.load_pdf_from_byte_vec(data, None)?;

    let render_config = PdfRenderConfig::new()
        .set_target_width(width as Pixels)
        .set_maximum_height(2000);

    for (i, page) in document.pages().iter().enumerate() {
        page.render_with_config(&render_config)?
            .as_image()
            .into_rgb8()
            .save_with_format(path.join(format!("{}-{:04}.png", name, i)), ImageFormat::Png)
            .map_err(|_| anyhow!("Failed to save image"))?;
    }
    Ok(document.pages().len()as _)
}

// 导出函数：从 LaTeX 生成 PDF 并转换为 PNG
#[unsafe(no_mangle)]
pub extern "C" fn latex_to_png(
    latex: *const c_char,
    name: *const c_char,
    path: *const c_char,
    width: c_uint,
    error: *mut *mut FfiError,
) -> c_int {
    let result = _latex_to_png(latex, name, path, width);
    match result {
        Ok(pages) => pages as c_int,
        Err(e) => {
            let ffi_error = Box::new(FfiError::new(e.to_string(), 1));
            unsafe {
                *error = Box::into_raw(ffi_error);
            }
            -1
        }
    }
}

fn _latex_to_png(
    latex: *const c_char,
    name: *const c_char,
    path: *const c_char,
    width: c_uint,
) -> Result<usize> {
    // 转换 C 字符串到 Rust 字符串
    let latex_cstr = unsafe { CStr::from_ptr(latex) };
    let latex_str = latex_cstr.to_str()?;
    
    let name_cstr = unsafe { CStr::from_ptr(name) };
    let name_str = name_cstr.to_str()?;
    
    let path_cstr = unsafe { CStr::from_ptr(path) };
    let path_str = path_cstr.to_str()?;
    let path_buf = PathBuf::from(path_str);
    
    // 调用原始函数
    render_to_png_batch(latex_str, name_str, &path_buf, width as u32)
}

fn render_to_png_batch(latex: &str, name: &str, path: &PathBuf, width: u32) -> Result<usize> {
    let pdf_data: Vec<u8> = tectonic::latex_to_pdf(latex).map_err(|e| anyhow!("{}", e.to_string()))?;
    export_pdf_to_png(pdf_data, name, path, width)
}