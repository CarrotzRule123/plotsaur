use piston_window::*;
use plotters_piston::draw_piston_window;

use super::chart::PlotChart;

pub struct PlotWindow {
    pub window: Option<PistonWindow>,
    pub chart: PlotChart
}

impl PlotWindow {
    pub fn new() -> Self {
        Self { 
            window: None,
            chart: PlotChart::new()
        }
    }

    pub fn open(&mut self, title: String, width: f64, height: f64) {
        self.window = Some(WindowSettings::new(title, [width, height])
            .samples(4)
            .build()
            .unwrap());
    }

    pub fn update(&mut self) -> usize {
        let mut control_flow = 0;
        if let Some(window) = &mut self.window {
            if let Some(events) = draw_piston_window(window, |b| {
                self.chart.draw(b);

                Ok(())
            }) {
                if let Some(_) = events.close_args() {
                    control_flow = 1;
                    self.window = None;
                };
            };
        }
        control_flow
    }
}
