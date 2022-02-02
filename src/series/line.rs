use plotters::prelude::*;

use serde::Deserialize;

use super::super::{ChartType, ShapeColor};
use super::SeriesType;

#[derive(Clone)]
pub struct LinePlot {
    pub color: ShapeColor,
    pub label: String,
    pub data: Vec<(f64, f64)>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LineOptions {
    pub color: ShapeColor,
    pub label: String,
}

impl LinePlot {
    pub fn build(options: LineOptions, buf: &[f64]) -> SeriesType {
        let mut data = Vec::new();
        for i in (0..buf.len()).step_by(2) {
            data.push((buf[i], buf[i + 1]))
        }
        SeriesType::Line(LinePlot {
            color: options.color,
            label: options.label,
            data,
        })
    }

    pub fn draw(&self, chart: &mut ChartType) {
        if let ChartType::Ranged(chart) = chart {
            let color = self.color.to_color();
            chart
                .draw_series(LineSeries::new(self.data.clone(), color.clone()))
                .expect("Error: Could not draw line!")
                .label(self.label.clone())
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color.clone()));
        }
    }
}