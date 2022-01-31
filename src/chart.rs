use plotters::coord::Shift;
use plotters::prelude::*;
use plotters_piston::PistonBackend;

use serde::Deserialize;

use super::types::{Range, ShapeColor, TextStyle};
use super::{ElementType, SeriesType};

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChartBuild {
    pub options: Vec<ChartOptions>,
}

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

pub struct PlotChart {
    pub options: Vec<ChartOptions>,
    pub elements: Vec<ElementType>,
    pub series: Vec<SeriesType>,
}

impl PlotChart {
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            elements: Vec::new(),
            series: Vec::new(),
        }
    }

    pub fn draw(&self, backend: PistonBackend) {
        let mut root = backend.into_drawing_area();
        root.fill(&WHITE).expect("Error: Could not fill window");
        {
            let mut builder = &mut ChartBuilder::on(&root);
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
                                .build_cartesian_2d(
                                    x_axis.start..x_axis.end,
                                    y_axis.start..y_axis.end,
                                )
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
            for series in &self.series {
                series.draw(&mut chart)
            }

            let mut series_label = chart.configure_series_labels();
            if let Some(options) = label_options {
                for option in options {
                    match option {
                        SeriesLabel::BackgroundStyle(style) => {
                            series_label.background_style(style.to_color())
                        }
                        SeriesLabel::BorderStyle(style) => {
                            series_label.border_style(style.to_color())
                        }
                    };
                }
                series_label.draw().expect("Error: Could not draw mesh!");
            };
        }
        for elements in &self.elements {
            elements.draw(&mut root)
        }
    }
}
