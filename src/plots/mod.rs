mod chart;

pub use chart::{ChartOptions, PlotChart};
use plotters::coord::Shift;
use plotters::prelude::*;
use plotters_piston::PistonBackend;

use serde::Deserialize;

pub enum PlotType {
    Chart(PlotChart),
    None,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
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
pub struct FontStyle {
    family: String,
    size: u32,
}
