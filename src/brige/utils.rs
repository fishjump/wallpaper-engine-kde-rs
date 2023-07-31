pub trait AsRawPtr {
    fn as_raw_ptr(&mut self) -> *mut Self {
        self as *mut Self
    }
}
