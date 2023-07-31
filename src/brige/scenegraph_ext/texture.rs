use crate::brige::utils::AsRawPtr;

pub enum Texture {}

impl AsRawPtr for Texture {}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(non_camel_case_types)]
pub enum Filtering {
    None = 0,
    Nearest = 1,
    Linear = 2,
}
