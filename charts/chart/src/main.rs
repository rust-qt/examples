#![windows_subsystem = "windows"]

use qt_charts::{QChart, QChartView, QLineSeries};
use qt_core::{qs, QTimer, SlotNoArgs};
use qt_gui::q_painter::RenderHint;
use qt_widgets::QApplication;

fn main() {
    QApplication::init(|_| unsafe {
        let series = QLineSeries::new_0a();
        series.set_name(&qs("Function"));
        for i in -500..500 {
            let x = i as f64 / 1000.0;
            let mut y = x * (1.0 / x).sin();
            if y.is_nan() {
                y = 0.0;
            }
            series.append_2_double(x, y);
        }

        let series2 = QLineSeries::new_0a();
        series2.set_name(&qs("Clock"));
        series2.append_2_double(0.0, 0.0);
        series2.append_2_double(1.0, 0.0);

        let chart = QChart::new_0a();
        chart.add_series(&series);
        chart.add_series(&series2);
        chart.create_default_axes();

        let chart_view = QChartView::from_q_chart(&chart);
        chart_view.set_window_title(&qs("Charts example"));
        chart_view.resize_2a(400, 300);
        chart_view.set_render_hint_1a(RenderHint::Antialiasing);
        chart_view.show();

        let timer = QTimer::new_0a();
        let mut var = 0.0f64;
        timer.timeout().connect(&SlotNoArgs::new(&timer, move || {
            var += 0.05;
            series2.clear();
            series2.append_2_double(0.0, 0.0);
            series2.append_2_double(var.cos() * 0.2, var.sin() * 0.2);
        }));
        timer.start_1a(100);

        QApplication::exec()
    })
}
