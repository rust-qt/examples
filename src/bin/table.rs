use qt_core::QString;
use qt_widgets::{
    QAction, QApplication, QMenu, QTableWidget, QTableWidgetItem,
    SlotOfQTableWidgetItemQTableWidgetItem,
};

fn main() {
    QApplication::init(|_| unsafe {
        let mut table = QTableWidget::new_0a();
        table.set_row_count(2);
        table.set_column_count(1);

        let mut item1 = QTableWidgetItem::new().into_ptr();
        item1.set_text(&QString::from_std_str("Item 1"));
        table.set_item(0, 0, item1);

        let mut item2 = QTableWidgetItem::new().into_ptr();
        item2.set_text(&QString::from_std_str("Item 2"));
        table.set_item(1, 0, item2);

        let slot = SlotOfQTableWidgetItemQTableWidgetItem::new(|mut current, mut previous| {
            if !previous.is_null() {
                let mut font = previous.font();
                font.set_bold(false);
                previous.set_font(&font);
            }
            if !current.is_null() {
                let mut font = current.font();
                font.set_bold(true);
                current.set_font(&font);
            }
            println!("ok");
        });
        table.current_item_changed().connect(&slot);
        table.show();

        let mut menu = QMenu::new();
        menu.add_action_q_string(&QString::from_std_str("A1"));
        menu.add_action_q_string(&QString::from_std_str("A2"));

        let action3 = QAction::from_q_string(&QString::from_std_str("A3")).into_ptr();
        menu.add_action(action3);

        menu.exec_0a_mut();

        QApplication::exec()
    })
}
