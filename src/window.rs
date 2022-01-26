use piston_window::*;
use plotters::prelude::*;
use plotters_piston::draw_piston_window;

pub struct PlotWindow {
    pub window: Option<PistonWindow>,
}

impl PlotWindow {
    pub fn new() -> Self {
        let window = Some(WindowSettings::new("Real Time CPU Usage", [450, 300])
            .samples(4)
            .build()
            .unwrap());
        Self { window }
    }

    pub fn update(&mut self) -> usize {
        let mut control_flow = 0;
        if let Some(window) = &mut self.window {
            if let Some(events) = draw_piston_window(window, |b| {
                let root = b.into_drawing_area();
                root.fill(&WHITE)?;
                let mut cc = ChartBuilder::on(&root)
                    .margin(10)
                    .caption("Real Time CPU Usage", ("sans-serif", 30))
                    .x_label_area_size(40)
                    .y_label_area_size(50)
                    .build_cartesian_2d(0..50 as u32, 0f32..1f32)?;
                cc.configure_mesh()
                    .x_labels(15)
                    .y_labels(5)
                    .x_desc("Seconds")
                    .y_desc("% Busy")
                    .axis_desc_style(("sans-serif", 15))
                    .draw()?;
                cc.configure_series_labels()
                    .background_style(&WHITE.mix(0.8))
                    .border_style(&BLACK)
                    .draw()?;

                Ok(())
            }) {
                if let Some(close_event) = events.close_args() {
                    control_flow = 1;
                    self.window = None;
                };
            };
        }
        control_flow
    }
}
