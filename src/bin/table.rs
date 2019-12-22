use qt_core::QString;
use qt_widgets::{
    QApplication, QTableWidget, QTableWidgetItem, SlotOfQTableWidgetItemQTableWidgetItem,
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

        QApplication::exec()
    })
}
