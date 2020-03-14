#![windows_subsystem = "windows"]

use qt_core::qs;
use qt_gui::QGuiApplication;
use qt_qml::QQmlApplicationEngine;

fn main() {
    QGuiApplication::init(|_| unsafe {
        let _engine = QQmlApplicationEngine::from_q_string(&qs("qrc:/main.qml"));
        QGuiApplication::exec()
    })
}
