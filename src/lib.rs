#![allow(unused_variables, dead_code, unused_imports)]

mod plot;
mod window;

use plot::{PlotProgram, PlotWidgets};
use window::PlotWindow;

use std::cell::RefCell;

pub struct Resources {
    widgets: PlotWidgets,
    window: PlotWindow,
}

impl Resources {
    pub fn new() -> Self {
        let widgets = PlotWidgets::new();
        let program = PlotProgram::new();
        let window = PlotWindow::new(program);
        Self { widgets, window }
    }
}

thread_local! {
    pub static RESOURCES: RefCell<Resources> = RefCell::new(Resources::new());
}

#[no_mangle]
pub extern "C" fn ops_create_window(ptr: *const u8, len: usize) {
    let buf = unsafe { std::slice::from_raw_parts(ptr, len) };
}

#[no_mangle]
pub extern "C" fn ops_run_return() {
    RESOURCES.with(|cell| {
        let mut resources = cell.borrow_mut();
        resources.window.run_return();
    })
}
