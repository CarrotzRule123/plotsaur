mod line;

use plotters::chart::ChartContext;
use plotters::coord::types::RangedCoordf64;
use plotters::prelude::Cartesian2d;
use plotters_piston::PistonBackend;

pub use line::*;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SeriesOptions {
    Line(LineOptions),
}

pub enum SeriesType {
    Line(Line),
}

impl SeriesType {
    pub fn build(options: SeriesOptions, buf: &[f64]) -> SeriesType {
        match options {
            SeriesOptions::Line(options) => Line::build(options, buf),
        }
    }

    pub fn draw(
        &self,
        chart: &mut ChartContext<PistonBackend, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    ) {
        match self {
            SeriesType::Line(line) => line.draw(chart),
        }
    }
}
