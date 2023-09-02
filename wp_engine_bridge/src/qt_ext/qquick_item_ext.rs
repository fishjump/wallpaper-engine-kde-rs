use cpp::cpp;
use qmetaobject::QQuickItem;

use super::qquick_window::QQuickWindow;

cpp! {{
    #include <QtQuick/QQuickItem>
}}

pub trait QQuickItemExt {
    fn set_flag(&mut self, flag: QQuickItemFlag);
    fn window(&self) -> &mut QQuickWindow;
}

impl<T: QQuickItem> QQuickItemExt for T {
    fn window(&self) -> &mut QQuickWindow {
        let item_ptr = self.get_cpp_object();
        cpp!(unsafe [item_ptr as "QQuickItem*"] -> &mut QQuickWindow as "QQuickWindow*" {
            return item_ptr->window();
        })
    }

    fn set_flag(&mut self, flag: QQuickItemFlag) {
        let obj = self.get_cpp_object();
        assert!(!obj.is_null());
        cpp!(unsafe [obj as "QQuickItem *", flag as "QQuickItem::Flag"] {
            obj->setFlag(flag);
        });
    }
}

#[repr(C)]
#[allow(unused)]
pub enum QQuickItemFlag {
    ItemClipsChildrenToShape = 0x01,
    ItemAcceptsInputMethod = 0x02,
    ItemIsFocusScope = 0x04,
    ItemHasContents = 0x08,
    ItemAcceptsDrops = 0x10,
}
