use piston_window::*;
use plotters::coord::Shift;
use plotters::prelude::*;
use plotters_piston::PistonBackend;

use serde::Deserialize;
use std::collections::vec_deque::VecDeque;

use super::{Range, ShapeColor, TextStyle};

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ChartOptions {
    Margin(u32),
    Caption {
        caption: String,
        style: TextStyle,
    },
    XLabelAreaSize(u32),
    YLabelAreaSize(u32),
    Cartesian2D {
        x_axis: Range<f64>,
        y_axis: Range<f64>,
    },
    Mesh(Vec<Mesh>),
    SeriesLabel(Vec<SeriesLabel>),
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Mesh {
    XLabels(usize),
    YLabels(usize),
    XDesc(String),
    YDesc(String),
    AxisDescStyle(TextStyle),
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SeriesLabel {
    BackgroundStyle(ShapeColor),
    BorderStyle(ShapeColor),
}

pub struct Series {
    pub color: ShapeColor,
    pub label: String,
    pub data: Vec<(f64, f64)>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SeriesOptions {
    pub color: ShapeColor,
    pub label: String,
}

pub struct PlotChart {
    pub options: Vec<ChartOptions>,
    pub data: Vec<Series>,
}

impl PlotChart {
    pub fn new(options: Vec<ChartOptions>) -> Self {
        Self {
            options,
            data: Vec::new(),
        }
    }

    pub fn plot(&mut self, buf: &[f64], options: SeriesOptions) {
        let mut data = Vec::new();
        for i in (0..buf.len()).step_by(2) {
            data.push((buf[i], buf[i + 1]))
        }
        self.data.push(Series {
            color: options.color,
            label: options.label,
            data
        })
    }

    pub fn draw(&self, root: &DrawingArea<PistonBackend, Shift>) {
        let mut builder = &mut ChartBuilder::on(root);
        let mut chart_builder = None;
        let mut mesh_options = None;
        let mut label_options = None;
        for option in &self.options {
            match option {
                ChartOptions::Margin(margin) => builder = builder.margin(margin.clone()),
                ChartOptions::Caption { caption, style } => {
                    builder = builder.caption(caption, (style.family.as_str(), style.size))
                }
                ChartOptions::XLabelAreaSize(size) => {
                    builder = builder.x_label_area_size(size.clone())
                }
                ChartOptions::YLabelAreaSize(size) => {
                    builder = builder.y_label_area_size(size.clone())
                }
                ChartOptions::Cartesian2D { x_axis, y_axis } => {
                    chart_builder = Some(
                        builder
                            .build_cartesian_2d(x_axis.start..x_axis.end, y_axis.start..y_axis.end)
                            .expect("Error: Could not build chart!"),
                    );
                }
                ChartOptions::Mesh(mesh) => mesh_options = Some(mesh),
                ChartOptions::SeriesLabel(label) => label_options = Some(label),
            }
        }

        let mut chart = chart_builder.unwrap();
        let mut mesh = chart.configure_mesh();
        if let Some(options) = mesh_options {
            for option in options {
                match option {
                    Mesh::XLabels(labels) => mesh.x_labels(labels.clone()),
                    Mesh::YLabels(labels) => mesh.y_labels(labels.clone()),
                    Mesh::XDesc(desc) => mesh.x_desc(desc),
                    Mesh::YDesc(desc) => mesh.y_desc(desc),
                    Mesh::AxisDescStyle(style) => {
                        mesh.axis_desc_style((style.family.as_str(), style.size))
                    }
                };
            }
            mesh.draw().expect("Error: Could not draw mesh!");
        };

        for series in &self.data {
            chart
                .draw_series(LineSeries::new(series.data.clone(), series.color.to_color()))
                .expect("Error: Could not draw series!")
                .label(series.label.clone())
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], series.color.to_color()));
        }

        let mut series_label = chart.configure_series_labels();
        if let Some(options) = label_options {
            for option in options {
                match option {
                    SeriesLabel::BackgroundStyle(style) => {
                        series_label.background_style(style.to_color())
                    }
                    SeriesLabel::BorderStyle(style) => series_label.border_style(style.to_color()),
                };
            }
            series_label.draw().expect("Error: Could not draw mesh!");
        };
    }
}
