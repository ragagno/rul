pub mod masks;
pub mod endian;

const BITS_PER_BYTE: usize = 8usize;

const BITS_PER_BYTE_U8: u8 = 8u8;
const BITS_PER_BYTE_U16: u16 = 8u16;
const BITS_PER_BYTE_U32: u32 = 8u32;
const BITS_PER_BYTE_U64: u64 = 8u64;
const BITS_PER_BYTE_U128: u128 = 8u128;

pub union UPunConst8 {
    pub u8: *const u8,
}

pub union UPunConst16 {
    pub u8: *const u8,
    pub u16: *const u16,
}

pub union UPunConst32 {
    pub u8: *const u8,
    pub u16: *const u16,
    pub u32: *const u32,
}

pub union UPunConst64 {
    pub u8: *const u8,
    pub u16: *const u16,
    pub u32: *const u32,
    pub u64: *const u64,
}

pub union UPunConst128 {
    pub u8: *const u8,
    pub u16: *const u16,
    pub u32: *const u32,
    pub u64: *const u64,
    pub u128: *const u128,
}

pub union UPunMut8 {
    pub u8: *mut u8,
}

pub union UPunMut16 {
    pub u8: *mut u8,
    pub u16: *mut u16,
}

pub union UPunMut32 {
    pub u8: *mut u8,
    pub u16: *mut u16,
    pub u32: *mut u32,
}

pub union UPunMut64 {
    pub u8: *mut u8,
    pub u16: *mut u16,
    pub u32: *mut u32,
    pub u64: *mut u64,
}

pub union UPun1Mut28 {
    pub u8: *mut u8,
    pub u16: *mut u16,
    pub u32: *mut u32,
    pub u64: *mut u64,
    pub u128: *mut u128,
}
