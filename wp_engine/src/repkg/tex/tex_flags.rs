use bitflags::bitflags;

bitflags! {
    #[derive(Debug)]
    pub struct TexFlags: u32 {
        const NONE = 0b0000_0000;               // 0
        const NO_INTERPOLATION = 0b0000_0001;   // 1
        const CLAMP_UVS = 0b0000_0010;          // 2
        const IS_GIF = 0b0000_0100;             // 4
        const UNK3 = 0b0000_1000;               // 8
        const UNK4 = 0b0001_0000;               // 16
        const UNK5 = 0b0010_0000;               // 32
        const UNK6 = 0b0100_0000;               // 64
        const UNK7 = 0b1000_0000;               // 128
    }
}
