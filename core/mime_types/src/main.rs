use qt_core::{qdebug, QCoreApplication, QListOfQString, QMimeDatabase};

fn main() {
    QCoreApplication::init(|_app| unsafe {
        let arguments = QCoreApplication::arguments();
        let db = QMimeDatabase::new();

        for arg in arguments.static_upcast::<QListOfQString>().iter() {
            let mime_type = db.mime_type_for_file_q_string(arg);
            println!("{:?}: {:?}", qdebug(arg), qdebug(mime_type.as_ref()));
        }
        0
    })
}
