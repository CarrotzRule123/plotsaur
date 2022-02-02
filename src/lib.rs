#![allow(unused_variables, dead_code, unused_imports)]

mod chart;
mod chart_type;
mod elements;
mod series;
mod types;
mod window;

pub use chart::*;
pub use chart_type::*;
pub use elements::*;
pub use series::*;
pub use types::*;
pub use window::*;

use std::cell::RefCell;

pub struct Resources {
    window: RefCell<PlotWindow>,
    dispatch_buf: RefCell<Option<&'static [u8]>>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            window: RefCell::new(PlotWindow::new()),
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
        let mut window = cell.window.borrow_mut();
        window.open(title.to_owned(), width, height);
    });
}

#[no_mangle]
pub extern "C" fn ops_build_plot(ptr: *const u8, len: usize) {
    let buf = unsafe { std::slice::from_raw_parts(ptr, len) };
    let json = std::str::from_utf8(&buf[0..len]).expect("Not a string");
    let options: ChartBuild = serde_json::from_str(json).unwrap();
    RESOURCES.with(|cell| {
        let mut window = cell.window.borrow_mut();
        window.chart.options = options.options.clone();
        for option in options.options {
            if let ChartOptions::Cartesian2D(options) = option {
                window.chart.chart_type = options.clone();
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn ops_draw_series(ptr: *const u8, len: usize, data: *const f64, data_len: usize) {
    let buf = unsafe { std::slice::from_raw_parts(ptr, len) };
    let json = std::str::from_utf8(&buf[0..len]).expect("Not a string");
    let options: SeriesOptions = serde_json::from_str(json).unwrap();
    let data = unsafe { std::slice::from_raw_parts(data, data_len) };
    // println!("{:?}", json);
    RESOURCES.with(|cell| {
        let mut window = cell.window.borrow_mut();
        let series = SeriesType::build(options, data, window.chart.chart_type.clone());
        window.chart.series.push(series);
    })
}

#[no_mangle]
pub extern "C" fn ops_draw_element(ptr: *const u8, len: usize) {
    let buf = unsafe { std::slice::from_raw_parts(ptr, len) };
    let json = std::str::from_utf8(&buf[0..len]).expect("Not a string");
    let element: ElementType = serde_json::from_str(json).unwrap();
    RESOURCES.with(|cell| {
        let mut window = cell.window.borrow_mut();
        window.chart.elements.push(element);
    })
}

#[no_mangle]
pub extern "C" fn ops_run_return() -> usize {
    RESOURCES.with(|cell| {
        let mut window = cell.window.borrow_mut();
        window.update()
    })
}
