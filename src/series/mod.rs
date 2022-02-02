mod histogram;
mod line;

pub use histogram::*;
pub use line::*;
use serde::Deserialize;

use super::{ChartType, ChartTypeOptions};

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SeriesOptions {
    Line(LineOptions),
    Histogram(HistogramOptions),
}

pub enum SeriesType {
    Line(LinePlot),
    Histogram(HistogramPlot),
}

impl SeriesType {
    pub fn build(options: SeriesOptions, buf: &[f64], chart_type: ChartTypeOptions) -> SeriesType {
        match options {
            SeriesOptions::Line(options) => LinePlot::build(options, buf),
            SeriesOptions::Histogram(options) => HistogramPlot::build(options, buf, chart_type),
        }
    }

    pub fn draw<'a>(&'a self, chart: &mut ChartType<'_, '_, 'a>) {
        match &self {
            SeriesType::Line(line) => line.draw(chart),
            SeriesType::Histogram(histogram) => histogram.draw(chart),
        }
    }
}
