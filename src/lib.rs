use std::sync::Mutex;

use deno_bindgen::deno_bindgen;
use fltk::{
    app::{self},
    enums::Mode,
    frame::{self, Frame},
    image::{self},
    prelude::*,
    window,
};
use lazy_static::lazy_static;

lazy_static! {
    static ref WINDOW: Mutex<Option<window::Window>> = Mutex::new(None);
    static ref APP: Mutex<Option<app::App>> = Mutex::new(None);
    static ref FRMAE: Mutex<Frame> = Mutex::new(frame::Frame::default());
    static ref DO_CLOSE: Mutex<bool> = Mutex::new(false);
}

#[deno_bindgen]
pub struct WindowOptions {
    label: String,
    resizeable: bool,
    width: i32,
    height: i32,
}

#[deno_bindgen]
fn create_window(options: WindowOptions) {
    let mut global = WINDOW.try_lock().unwrap();
    *global = Some(window::Window::default().with_size(options.width, options.height));
    let local = global.as_mut().unwrap();
    local.set_callback(move |_| {
        let mut do_close = DO_CLOSE.try_lock().unwrap();
        *do_close = true;
    });
    local.make_resizable(options.resizeable);
    local.set_label(options.label.trim());
    local.set_visible_focus();
}

#[deno_bindgen]
fn push_png_buffer(buffer: &'static [u8]) {
    let mut frame = FRMAE.try_lock().unwrap();
    let image = image::PngImage::from_data(buffer).unwrap();
    frame.set_image(Some(image));
    frame.parent().unwrap().redraw();
}

#[deno_bindgen]
fn push_jpeg_buffer(buffer: &'static [u8]) {
    let mut frame = FRMAE.try_lock().unwrap();
    let image = image::JpegImage::from_data(buffer).unwrap();
    frame.set_image(Some(image));
    frame.parent().unwrap().redraw();
}

#[deno_bindgen]
fn show_window() {
    let mut mut_window = WINDOW.try_lock().unwrap();
    let local = mut_window.as_mut().unwrap();
    let mut var_frame = FRMAE.try_lock().unwrap();
    local.end();
    local.show();
    var_frame.set_size(local.width(), local.height());
}

#[deno_bindgen]
fn quit() {
    app::quit();
    let global = APP.try_lock().unwrap();
    global.unwrap().quit();
}

#[deno_bindgen]
fn update_loop() -> u8 {
    app::wait_for(0.0001).unwrap();

    let global = DO_CLOSE.try_lock().unwrap();
    if *global {
        1
    } else {
        0
    }
}

#[deno_bindgen]
fn quick_wait() {
    app::wait_for(0.000001).unwrap();
}

#[deno_bindgen]
fn init() {
    let mut global = APP.try_lock().unwrap();
    *global = Some(app::App::default());
    // let var_name = global.as_mut().unwrap();
    // var_name
    // .set_visual(Mode::Rgb | Mode::Opengl3 | Mode::Stereo | Mode::Stencil)
    // .unwrap();
}
