use cpp_core::Ptr;
use qt_core::{qs, CheckState, QBox, QPtr, SlotNoArgs};
use qt_gui::{QStandardItem, QStandardItemModel};
use qt_ui_tools::QUiLoader;
use qt_widgets::{QApplication, QListView, QPushButton, QRadioButton, QWidget};
use std::rc::Rc;

#[derive(Debug)]
struct Form {
    widget: QBox<QWidget>,
    add: QPtr<QPushButton>,
    remove_selected: QPtr<QPushButton>,
    list: QPtr<QListView>,
    show_all: QPtr<QRadioButton>,
    show_active: QPtr<QRadioButton>,
    show_completed: QPtr<QRadioButton>,
    remove_completed: QPtr<QPushButton>,
}

impl Form {
    pub fn load() -> Form {
        unsafe {
            let loader = QUiLoader::new_0a();
            let widget = loader.load_bytes(include_bytes!("../ui/form.ui"));
            assert!(!widget.is_null(), "invalid ui file");

            Form {
                add: widget.find_child("add").unwrap(),
                remove_selected: widget.find_child("remove_selected").unwrap(),
                remove_completed: widget.find_child("remove_completed").unwrap(),
                list: widget.find_child("list").unwrap(),
                show_all: widget.find_child("show_all").unwrap(),
                show_active: widget.find_child("show_active").unwrap(),
                show_completed: widget.find_child("show_completed").unwrap(),
                widget,
            }
        }
    }
}

#[derive(Debug)]
struct TodoWidget {
    form: Form,
    model: QBox<QStandardItemModel>,
}

impl TodoWidget {
    fn new() -> Rc<Self> {
        let this = Rc::new(TodoWidget {
            form: Form::load(),
            model: unsafe { QStandardItemModel::new_0a() },
        });
        this.init();
        this
    }

    fn init(self: &Rc<Self>) {
        unsafe {
            self.form.list.set_model(&self.model);
            self.form.add.clicked().connect(&self.slot_on_add_clicked());
            self.form
                .list
                .selection_model()
                .selection_changed()
                .connect(&self.slot_on_list_selection_changed());
            self.on_list_selection_changed();
        }
    }

    #[inline]
    unsafe fn main_widget(&self) -> Ptr<QWidget> {
        self.form.widget.as_ptr()
    }

    unsafe fn slot_on_add_clicked(self: &Rc<Self>) -> QBox<SlotNoArgs> {
        let this = Rc::clone(&self);
        SlotNoArgs::new(self.main_widget(), move || {
            this.on_add_clicked();
        })
    }

    fn on_add_clicked(self: &Rc<Self>) {
        unsafe {
            let item = QStandardItem::new().into_ptr();
            item.set_text(&qs("New item"));
            item.set_check_state(CheckState::Unchecked);
            item.set_checkable(true);
            self.model.append_row_q_standard_item(item);
            self.form.list.set_current_index(&item.index());
            self.form.list.edit(&item.index());
        }
    }

    unsafe fn slot_on_list_selection_changed(self: &Rc<Self>) -> QBox<SlotNoArgs> {
        let this = Rc::clone(&self);
        SlotNoArgs::new(self.main_widget(), move || {
            this.on_list_selection_changed();
        })
    }

    fn on_list_selection_changed(self: &Rc<Self>) {
        unsafe {
            let count = self
                .form
                .list
                .selection_model()
                .selected_rows_0a()
                .count_0a();
            self.form.remove_selected.set_enabled(count > 0);
        }
    }

    fn show(self: &Rc<Self>) {
        unsafe {
            self.form.widget.show();
        }
    }
}

fn main() {
    QApplication::init(|_| {
        let todo_widget = TodoWidget::new();
        todo_widget.show();
        unsafe { QApplication::exec() }
    })
}

#[cfg(test)]
mod tests {}
