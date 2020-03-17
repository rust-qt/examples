#![windows_subsystem = "windows"]

use cpp_core::{Ptr, StaticUpcast};
use qt_core::{
    q_init_resource, qs, slot, CheckState, ItemDataRole, QBox, QObject, QPtr,
    QSortFilterProxyModel, SlotNoArgs,
};
use qt_gui::{QStandardItem, QStandardItemModel};
use qt_ui_tools::ui_form;
use qt_widgets::{QApplication, QListView, QPushButton, QRadioButton, QWidget};
use std::collections::BTreeSet;
use std::rc::Rc;

#[ui_form("../ui/form.ui")]
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

#[derive(Debug)]
struct TodoWidget {
    form: Form,
    source_model: QBox<QStandardItemModel>,
    proxy_model: QBox<QSortFilterProxyModel>,
}

impl StaticUpcast<QObject> for TodoWidget {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.form.widget.as_ptr().static_upcast()
    }
}

impl TodoWidget {
    fn new() -> Rc<Self> {
        unsafe {
            let this = Rc::new(TodoWidget {
                form: Form::load(),
                source_model: QStandardItemModel::new_0a(),
                proxy_model: QSortFilterProxyModel::new_0a(),
            });
            this.init();
            this
        }
    }

    unsafe fn init(self: &Rc<Self>) {
        for &(text, is_done) in &[
            ("Learn Qt", true),
            ("Learn Rust", true),
            ("Conquer the world", false),
        ] {
            let item = QStandardItem::new().into_ptr();
            item.set_text(&qs(text));
            item.set_checkable(true);
            item.set_check_state(if is_done {
                CheckState::Checked
            } else {
                CheckState::Unchecked
            });
            self.source_model.append_row_q_standard_item(item);
        }

        self.proxy_model.set_source_model(&self.source_model);
        self.proxy_model
            .set_filter_role(ItemDataRole::CheckStateRole.into());
        self.form.list.set_model(&self.proxy_model);
        self.form.add.clicked().connect(&self.slot_on_add_clicked());
        self.form
            .remove_selected
            .clicked()
            .connect(&self.slot_on_remove_selected_clicked());
        self.form
            .remove_completed
            .clicked()
            .connect(&self.slot_on_remove_completed_clicked());
        self.form
            .list
            .selection_model()
            .selection_changed()
            .connect(&self.slot_on_list_selection_changed());

        for button in &[
            &self.form.show_completed,
            &self.form.show_active,
            &self.form.show_all,
        ] {
            button.toggled().connect(&self.slot_on_filter_changed());
        }

        self.on_list_selection_changed();
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_add_clicked(self: &Rc<Self>) {
        if self.form.show_completed.is_checked() {
            self.form.show_all.set_checked(true);
        }

        let item = QStandardItem::new().into_ptr();
        item.set_text(&qs("New item"));
        item.set_checkable(true);
        item.set_check_state(CheckState::Unchecked);
        self.source_model.append_row_q_standard_item(item);
        let index = self.proxy_model.map_from_source(&item.index());
        self.form.list.set_current_index(&index);
        self.form.list.edit(&index);
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_list_selection_changed(self: &Rc<Self>) {
        let count = self
            .form
            .list
            .selection_model()
            .selected_rows_0a()
            .count_0a();
        self.form.remove_selected.set_enabled(count > 0);
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_filter_changed(self: &Rc<Self>) {
        let filter_value = if self.form.show_active.is_checked() {
            Some(CheckState::Unchecked)
        } else if self.form.show_completed.is_checked() {
            Some(CheckState::Checked)
        } else {
            None
        };
        let filter_string = if let Some(filter_value) = filter_value {
            filter_value.to_int().to_string()
        } else {
            String::new()
        };
        self.proxy_model.set_filter_fixed_string(&qs(filter_string));
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_remove_selected_clicked(self: &Rc<Self>) {
        let selection = self
            .proxy_model
            .map_selection_to_source(&self.form.list.selection_model().selection());
        let rows = selection
            .indexes()
            .iter()
            .map(|index| index.row())
            .collect::<BTreeSet<_>>();
        for &row in rows.iter().rev() {
            self.source_model.remove_row_1a(row);
        }
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_remove_completed_clicked(self: &Rc<Self>) {
        for row in (0..self.source_model.row_count_0a()).rev() {
            let state = self.source_model.data_2a(
                &self.source_model.index_2a(row, 0),
                ItemDataRole::CheckStateRole.into(),
            );
            if state.to_int_0a() == CheckState::Checked.to_int() {
                self.source_model.remove_row_1a(row);
            }
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
        q_init_resource!("resources");
        let todo_widget = TodoWidget::new();
        todo_widget.show();
        unsafe { QApplication::exec() }
    })
}
