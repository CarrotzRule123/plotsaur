pub mod chart;

pub use chart::{ChartOptions, PlotChart};
use plotters::coord::Shift;
use plotters::prelude::*;
use plotters_piston::PistonBackend;
use plotters::style::RGBAColor;

use serde::Deserialize;

pub enum PlotType {
    Chart(PlotChart),
    None,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PlotOptions {
    Chart(Vec<ChartOptions>),
    None,
}

impl PlotType {
    pub fn new(options: PlotOptions) -> Self {
        match options {
            PlotOptions::Chart(options) => {
                PlotType::Chart(PlotChart::new(options))
            }
            _ => PlotType::None,
        }
    }

    pub fn draw(&self, root: &DrawingArea<PistonBackend, Shift>) {
        match self {
            PlotType::Chart(chart) => chart.draw(root),
            _ => (),
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct TextStyle {
    family: String,
    size: u32,
}

#[derive(Deserialize, Clone)]
pub struct Point {
    x: u32,
    y: u32,
}

#[derive(Deserialize, Clone)]
pub struct Range<T> {
    start: T,
    end: T,
}

#[derive(Deserialize, Clone)]
pub struct ShapeColor {
    r: u8,
    g: u8,
    b: u8,
    a: f64,
}

impl ShapeColor {
    pub fn to_color(&self) -> RGBAColor {
        RGBColor(self.r, self.g, self.b).mix(self.a)
    }
}