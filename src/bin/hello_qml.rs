#![windows_subsystem = "windows"]

use qt_core::QString;
use qt_gui::QGuiApplication;
use qt_qml::QQmlApplicationEngine;

fn main() {
    QGuiApplication::init(|_| unsafe {
        let main_qml_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/hello_qml/main.qml");
        let _engine = QQmlApplicationEngine::from_q_string(&QString::from_std_str(main_qml_path));
        QGuiApplication::exec()
    })
}
