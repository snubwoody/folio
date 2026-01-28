use qmetaobject::{QmlEngine, qml_register_type};

fn main() {
    let mut engine = QmlEngine::new();
    engine.load_file("crates/folio-qt/ui/App.qml".into());
    engine.exec();
}
