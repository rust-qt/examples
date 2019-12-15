#![windows_subsystem = "windows"]

use qt_core::{QBuffer, QByteArray, QString, SlotOfBool};
use qt_ui_tools::{cpp_core::CppBox, QUiLoader};
use qt_widgets::{QApplication, QCheckBox, QLabel, QWidget};

struct Form {
    _widget: CppBox<QWidget>,
    _check_box_toggled: SlotOfBool<'static>,
}

impl Form {
    fn new() -> Form {
        unsafe {
            let form_data = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/form1.ui"));
            let mut byte_array = QByteArray::from_slice(form_data);
            let mut buffer = QBuffer::from_q_byte_array(&mut byte_array);
            let mut ui_loader = QUiLoader::new_0a();
            let mut widget = CppBox::new(ui_loader.load_1a(&mut buffer)).expect("load failed");
            widget.show();

            let check_box = widget
                .find_child_q_object_1a(&QString::from_std_str("checkBox"))
                .as_mut_ref()
                .expect("child not found")
                .dynamic_cast_mut::<QCheckBox>()
                .expect("widget type mismatch");

            let mut label = widget
                .find_child_q_object_1a(&QString::from_std_str("label"))
                .as_mut_ref()
                .expect("child not found")
                .dynamic_cast_mut::<QLabel>()
                .expect("widget type mismatch");

            let check_box_toggled = SlotOfBool::new(move |checked| {
                let text = if checked { "Checked!" } else { "Unchecked!" };
                label.set_text(&QString::from_std_str(text));
            });
            check_box.toggled().connect(&check_box_toggled);

            Form {
                _widget: widget,
                _check_box_toggled: check_box_toggled,
            }
        }
    }
}

fn main() {
    QApplication::init(|_| unsafe {
        let _form = Form::new();
        QApplication::exec()
    })
}
