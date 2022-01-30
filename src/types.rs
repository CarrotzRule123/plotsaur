use plotters::coord::Shift;
use plotters::prelude::*;
use plotters::style::RGBAColor;
use plotters_piston::PistonBackend;

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct TextStyle {
    pub family: String,
    pub size: u32,
}

#[derive(Deserialize, Clone)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

#[derive(Deserialize, Clone)]
pub struct Range<T> {
    pub start: T,
    pub end: T,
}

#[derive(Deserialize, Clone)]
pub struct ShapeColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f64,
}

pub struct BackendColor {
    pub alpha: f64,
    pub rgb: (u8, u8, u8),
}

impl ShapeColor {
    pub fn to_color(&self) -> RGBAColor {
        RGBColor(self.r, self.g, self.b).mix(self.a)
    }
}
