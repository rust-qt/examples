use qt_core::{qs, CheckState, QBox, QMutPtr, SlotNoArgs};
use qt_gui::{QStandardItem, QStandardItemModel};
use qt_ui_tools::QUiLoader;
use qt_widgets::{QApplication, QListView, QPushButton, QWidget};

#[derive(Debug, Clone)]
struct Form {
    widget: QMutPtr<QWidget>,
    add: QMutPtr<QPushButton>,
    remove: QMutPtr<QPushButton>,
    list: QMutPtr<QListView>,
}

impl Form {
    pub fn load() -> (QBox<QWidget>, Form) {
        unsafe {
            let mut loader = QUiLoader::new_0a();
            let mut widget = loader.load_bytes(include_bytes!("../ui/form.ui"));
            assert!(!widget.is_null(), "invalid ui file");

            let form = Form {
                add: widget.find_child("add").unwrap(),
                remove: widget.find_child("remove").unwrap(),
                list: widget.find_child("list").unwrap(),
                widget: QMutPtr::new(widget.as_mut_ptr()),
            };
            (widget, form)
        }
    }
}

fn main() {
    QApplication::init(|_| unsafe {
        let (mut widget, mut form) = Form::load();
        let mut model = QStandardItemModel::new_0a();
        form.list.set_model(&mut model);
        let mut model_ptr = model.as_mut_ptr();

        let mut form1 = form.clone();
        form.add
            .clicked()
            .connect(&SlotNoArgs::new(&mut widget, move || {
                let mut item = QStandardItem::new().into_ptr();
                item.set_text(&qs("New item"));
                item.set_check_state(CheckState::Unchecked);
                item.set_checkable(true);
                model_ptr.append_row_q_standard_item(item);
                form1.list.set_current_index(&item.index());
                form1.list.edit(&item.index());
            }));

        let mut form1 = form.clone();
        let mut on_selection_changed = SlotNoArgs::new(&mut widget, move || {
            let count = form1.list.selection_model().selected_rows_0a().count_0a();
            form1.remove.set_enabled(count > 0);
        });
        form.list
            .selection_model()
            .selection_changed()
            .connect(&on_selection_changed);
        on_selection_changed.slot();

        form.widget.show();
        QApplication::exec()
    })
}

#[cfg(test)]
mod tests {
    use qt_core::{QBox, QObject};

    #[test]
    fn qbox1() {
        unsafe {
            let mut obj = QBox::new(QObject::new_0a().into_ptr());
            assert_eq!(obj.children().length(), 0);
            {
                let _obj2 = QBox::new(QObject::new_1a(&mut obj).into_ptr());
                assert_eq!(obj.children().length(), 1);
            }
            assert_eq!(obj.children().length(), 1);
        }
    }

    #[test]
    fn qbox2() {
        unsafe {
            let mut obj = QBox::new(QObject::new_0a().into_ptr());
            let obj2 = QBox::new(QObject::new_1a(&mut obj).into_ptr());
            assert!(!obj2.is_null());
            drop(obj);
            assert!(obj2.is_null());
        }
    }

    #[test]
    fn qbox3() {
        unsafe {
            let mut obj = QBox::new(QObject::new_0a().into_ptr());
            let obj2 = QBox::new(QObject::new_1a(&mut obj).into_ptr());
            assert!(!obj2.is_null());
            let _obj1 = obj.into_q_ptr();
            assert!(!obj2.is_null());
        }
    }
}
