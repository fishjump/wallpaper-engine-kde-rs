use cstr::cstr;
use qmetaobject::prelude::*;
use std::ffi::CStr;

// The `QObject` custom derive macro allows to expose a class to Qt and QML
#[derive(QObject, Default)]
struct Greeter {
    // Specify the base class with the qt_base_class macro
    base: qt_base_class!(trait QObject),
    // Declare `name` as a property usable from Qt
    name: qt_property!(QString; NOTIFY name_changed),
    // Declare a signal
    name_changed: qt_signal!(),
    // And even a slot
    compute_greetings: qt_method!(
        fn compute_greetings(&self, verb: String) -> QString {
            format!("{}, {}!", verb, self.name.to_string()).into()
        }
    ),
}

#[derive(Default, QObject)]
struct QExampleQmlPlugin {
    base: qt_base_class!(trait QQmlExtensionPlugin),
    plugin: qt_plugin!("org.qt-project.Qt.QQmlExtensionInterface/1.0"),
}

impl QQmlExtensionPlugin for QExampleQmlPlugin {
    fn register_types(&mut self, uri: &CStr) {
        qml_register_type::<Greeter>(uri, 1, 0, cstr!("Greeter"));
    }
}
