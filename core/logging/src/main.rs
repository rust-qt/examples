#![windows_subsystem = "windows"]

use qt_core::{
    q_critical, q_debug, q_info, q_set_message_pattern, q_warning, qs, QCoreApplication,
};

fn main() {
    QCoreApplication::init(|_app| unsafe {
        let _ = q_debug!() << qs("Hello World!").as_ref();

        q_set_message_pattern(&qs("%{file}:%{line} [%{function}] %{type}: %{message}"));

        let _ = q_debug!() << qs("Example debug").as_ref();
        let _ = q_info!() << qs("Example info").as_ref() << 1i64;
        let _ = q_warning!() << qs("Example warning").as_ref();
        let _ = q_critical!() << qs("Example critical").as_ref();
        0
    })
}
