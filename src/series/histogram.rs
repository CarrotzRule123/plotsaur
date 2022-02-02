use plotters::chart::ChartContext;
use plotters::coord::ranged1d::SegmentedCoord;
use plotters::coord::types::{RangedCoordf64, RangedCoordi32, RangedSlice};
use plotters::prelude::*;
use plotters_piston::PistonBackend;

use serde::Deserialize;

use super::super::{ChartType, ChartTypeOptions, PlotChart, Range, ShapeColor};
use super::SeriesType;
use std::collections::HashMap;

pub enum Values {
    Segmented { range: Range<i32>, data: Vec<f64> },
    Values(Vec<(String, f64)>),
}

pub struct HistogramPlot {
    pub color: ShapeColor,
    pub filled: bool,
    pub values: Values,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HistogramOptions {
    pub color: ShapeColor,
    pub filled: bool,
}

impl HistogramPlot {
    pub fn build(
        options: HistogramOptions,
        buf: &[f64],
        chart_type: ChartTypeOptions,
    ) -> SeriesType {
        let values = match chart_type {
            ChartTypeOptions::SegmentedX { x_axis, .. } => Values::Segmented {
                range: x_axis,
                data: buf.to_vec(),
            },
            ChartTypeOptions::SegmentedY { y_axis, .. } => Values::Segmented {
                range: y_axis,
                data: buf.to_vec(),
            },
            ChartTypeOptions::ValuesX { x_axis, .. } => {
                let mut values = Vec::new();
                for i in 0..x_axis.len() {
                    values.push((x_axis[i].clone(), buf[i]));
                }
                Values::Values(values)
            }
            ChartTypeOptions::ValuesY { y_axis, .. } => {
                let mut values = Vec::new();
                for i in 0..y_axis.len() {
                    values.push((y_axis[i].clone(), buf[i]));
                }
                Values::Values(values)
            }
            _ => panic!("Error: Could not draw histogram!"),
        };
        SeriesType::Histogram(HistogramPlot {
            color: options.color,
            filled: options.filled,
            values: values,
        })
    }

    pub fn draw<'a>(&'a self, chart: &mut ChartType<'_, '_, 'a>) {
        let mut style: ShapeStyle = self.color.to_color().into();
        if self.filled {
            style = style.filled();
        };
        match chart {
            ChartType::SegmentedX(chart) => {
                if let Values::Segmented { range, data } = &self.values {
                    chart
                        .draw_series((range.start..range.end).zip(data.iter()).map(|(x, y)| {
                            let x0 = SegmentValue::Exact(x);
                            let x1 = SegmentValue::Exact(x + 1);
                            let mut bar = Rectangle::new([(x0, 0.0), (x1, *y)], style.clone());
                            bar.set_margin(0, 0, 5, 5);
                            bar
                        }))
                        .expect("Error: Could not draw histogram!");
                };
            }
            ChartType::SegmentedY(chart) => {
                if let Values::Segmented { range, data } = &self.values {
                    chart
                        .draw_series((range.start..range.end).zip(data.iter()).map(|(y, x)| {
                            let y0 = SegmentValue::Exact(y);
                            let y1 = SegmentValue::Exact(y + 1);
                            let mut bar = Rectangle::new([(0.0, y0), (*x, y1)], style.clone());
                            bar.set_margin(5, 5, 0, 0);
                            bar
                        }))
                        .expect("Error: Could not draw histogram!");
                };
            }
            ChartType::ValuesX(chart) => {
                if let Values::Values(data) = &self.values {
                    chart
                        .draw_series(
                            Histogram::vertical(&chart)
                                .margin(100)
                                .style(style.clone())
                                .data(data.iter().map(|(x, y)| (x, y.clone()))),
                        )
                        .expect("Error: Could not draw histogram!");
                };
            }
            ChartType::ValuesY(chart) => {
                if let Values::Values(data) = &self.values {
                    chart
                        .draw_series(
                            Histogram::horizontal(&chart)
                                .margin(100)
                                .style(style.clone())
                                .data(data.iter().map(|(x, y)| (x, y.clone()))),
                        )
                        .expect("Error: Could not draw histogram!");
                };
            }
            _ => panic!("Unreachable!"),
        }
    }
}
