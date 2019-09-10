use qt_charts::{QChart, QChartView, QLineSeries};
use qt_core::QString;
use qt_widgets::QApplication;

fn main() {
    QApplication::init(|_| unsafe {
        let mut series = QLineSeries::new_0a();
        series.set_name(&QString::from_std_str("Data"));
        series.append_2_double(1.0, 1.0);
        series.append_2_double(2.0, 5.0);
        series.append_2_double(3.0, 2.0);
        series.append_2_double(5.0, 7.0);
        series.append_2_double(7.0, 1.0);

        let mut chart = QChart::new_0a();
        chart.add_series(series.into_ptr());
        chart.create_default_axes();

        let mut chart_view = QChartView::from_q_chart(chart.into_ptr());
        chart_view.set_window_title(&QString::from_std_str("Charts example"));
        chart_view.resize_2a(400, 300);
        chart_view.show();
        QApplication::exec()
    })
}
