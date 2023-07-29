pub enum Texture {}

impl Into<*mut Texture> for &mut Texture {
    fn into(self) -> *mut Texture {
        self as *mut Texture
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(non_camel_case_types)]
pub enum Filtering {
    None = 0,
    Nearest = 1,
    Linear = 2,
}
