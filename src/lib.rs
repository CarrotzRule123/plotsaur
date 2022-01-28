#![allow(unused_variables, dead_code, unused_imports)]

mod plots;
mod window;

use plots::{PlotOptions, PlotType};
use window::PlotWindow;

use std::cell::RefCell;

pub struct Resources {
    window: RefCell<Option<PlotWindow>>,
    dispatch_buf: RefCell<Option<&'static [u8]>>
}

impl Resources {
    pub fn new() -> Self {
        Self {
            window: RefCell::new(Some(PlotWindow::new())),
            dispatch_buf: RefCell::new(None),
        }
    }
}

thread_local! {
    pub static RESOURCES: Resources = Resources::new();
}

#[no_mangle]
pub extern "C" fn ops_create_window(ptr: *const u8, len: usize, width: f64, height: f64) {
    let buf = unsafe { std::slice::from_raw_parts(ptr, len) };
    let title = std::str::from_utf8(&buf[0..len]).expect("Not a string");
    RESOURCES.with(|cell| {
        if let Some(window) = cell.window.borrow_mut().as_mut() {
            window.open(title.to_owned(), width, height);
        };
        // cell.dispatch_buf.replace(Some(buf));
    });
}

#[no_mangle]
pub extern "C" fn ops_build_plot(ptr: *const u8, len: usize) {
    let buf = unsafe { std::slice::from_raw_parts(ptr, len) };
    let json = std::str::from_utf8(&buf[0..len]).expect("Not a string");
    let options: PlotOptions = serde_json::from_str(json).unwrap();
    RESOURCES.with(|cell| {
        if let Some(window) = cell.window.borrow_mut().as_mut() {
            window.plot = PlotType::new(options);
        };
    })
}

#[no_mangle]
pub extern "C" fn ops_run_return() -> usize {
    RESOURCES.with(|cell| {
        if let Some(window) = cell.window.borrow_mut().as_mut() {
            return window.update();
        }
        0
    })
}
