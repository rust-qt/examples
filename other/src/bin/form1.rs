#![windows_subsystem = "windows"]

use qt_core::{QBox, QString, SlotNoArgs};
use qt_widgets::{QApplication, QLineEdit, QMessageBox, QPushButton, QVBoxLayout, QWidget};

fn form() -> QBox<QWidget> {
    unsafe {
        let mut widget = QWidget::new_0a();
        let mut layout = QVBoxLayout::new_1a(&mut widget).into_ptr();
        let mut line_edit = QLineEdit::new();

        layout.add_widget(&mut line_edit);
        let line_edit = line_edit.into_ptr();

        let mut button = QPushButton::from_q_string(&QString::from_std_str("Start"));
        button.set_enabled(false);

        layout.add_widget(&mut button);
        let mut button = button.into_ptr();

        widget.show();
        let widget_ptr = widget.as_mut_ptr();

        button.clicked().connect(&SlotNoArgs::new(button, move || {
            let text = line_edit.text();
            QMessageBox::information_q_widget2_q_string(
                widget_ptr,
                &QString::from_std_str("My title"),
                &QString::from_std_str("Text: \"%1\". Congratulations!").arg_q_string(&text),
            );
        }));
        line_edit
            .text_edited()
            .connect(&SlotNoArgs::new(line_edit, move || {
                button.set_enabled(!line_edit.text().is_empty());
            }));
        widget
    }
}

fn main() {
    QApplication::init(|_| unsafe {
        let _form = form();
        QApplication::exec()
    })
}
