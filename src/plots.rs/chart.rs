struct MyChart {
    width: u16,
    height: u16,
}

impl MyChart {
    pub fn new() -> Self {
        Self {
            width: 800,
            height: 600,
        }
    }

    fn view(&mut self) -> Element<Message, Renderer> {
        let chart = ChartWidget::new(self)
            .width(Length::Fill)
            .height(Length::Fill);

        chart.into()
    }
}

impl Chart<Message> for MyChart {
    // leave it empty
    fn build_chart<DB: DrawingBackend>(&self, _builder: ChartBuilder<DB>) {}

    fn draw_chart<DB: DrawingBackend>(&self, root: DrawingArea<DB, Shift>) {
        let mut builder = ChartBuilder::on(&root);
        let mut chart = builder
            .margin(30)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-10f32..10f32, -10f32..10f32)
            .unwrap();

        chart
            .configure_mesh()
            .x_labels(10)
            .y_labels(10)
            .draw()
            .unwrap();

        chart
            .draw_series(LineSeries::new(
                (-500..=500)
                    .map(|x| x as f32 / 50.0)
                    .map(|x| (x, x.powf(2.0))),
                &RED,
            ))
            .unwrap();
    }
}