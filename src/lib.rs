#![allow(unused_variables, dead_code, unused_imports)]

// mod plot;
mod window;

// use plot::{PlotProgram, PlotWidgets};
use window::PlotWindow;

use std::cell::RefCell;

pub struct Resources {
    // widgets: RefCell<PlotWidgets>,
    window: RefCell<Option<PlotWindow>>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            // widgets: RefCell::new(PlotWidgets::new()),
            window: RefCell::new(None),
        }
    }
}

thread_local! {
    pub static RESOURCES: Resources = Resources::new();
}

#[no_mangle]
pub extern "C" fn ops_create_window(ptr: *const u8, len: usize) {
    let buf = unsafe { std::slice::from_raw_parts(ptr, len) };
    let title = std::str::from_utf8(buf).expect("Not a string").to_owned();
    let window = PlotWindow::new();
    RESOURCES.with(|cell| {
        cell.window.replace(Some(window));
    });
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
