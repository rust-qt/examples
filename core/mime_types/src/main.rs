use qt_core::{qdbg, QCoreApplication, QListOfQString, QMimeDatabase};

fn main() {
    QCoreApplication::init(|_app| unsafe {
        let arguments = QCoreApplication::arguments();
        let db = QMimeDatabase::new();

        for arg in arguments.static_upcast::<QListOfQString>().iter() {
            let mime_type = db.mime_type_for_file_q_string(arg);
            println!("{:?}: {:?}", qdbg(arg), qdbg(mime_type.as_ref()));
        }
        0
    })
}
