use std::ffi::CStr;

use cstr::cstr;
use qmetaobject::prelude::*;

use crate::greeter::Greeter;
use crate::renderer;

const MAJOR_VERSION: u32 = 1;
const MINOR_VERSION: u32 = 0;

#[derive(Default, QObject)]
struct WallpaperEngineKdePlugin {
    base: qt_base_class!(trait QQmlExtensionPlugin),
    plugin: qt_plugin!("org.qt-project.Qt.QQmlExtensionInterface/1.0"),
}

impl QQmlExtensionPlugin for WallpaperEngineKdePlugin {
    fn register_types(&mut self, uri: &CStr) {
        qml_register_type::<Greeter>(
            uri,
            MAJOR_VERSION,
            MINOR_VERSION,
            cstr!("Greeter"),
        );
        qml_register_type::<renderer::scene_renderer::SceneRenderer>(
            uri,
            MAJOR_VERSION,
            MINOR_VERSION,
            cstr!("SceneRenderer"),
        );
    }
}
