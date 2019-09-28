pub mod masks;

const BITS_PER_BYTE: usize = 8usize;

pub union UPunConst8 {
    u8: *const u8,
}

pub union UPunConst16 {
    u8: *const u8,
    u16: *const u16,
}

pub union UPunConst32 {
    u8: *const u8,
    u16: *const u16,
    u32: *const u32,
}

pub union UPunConst64 {
    u8: *const u8,
    u16: *const u16,
    u32: *const u32,
    u64: *const u64,
}

pub union UPunConst128 {
    u8: *const u8,
    u16: *const u16,
    u32: *const u32,
    u64: *const u64,
    u128: *const u128,
}

pub union UPuMutn8 {
    u8: *mut u8,
}

pub union UPunMut16 {
    u8: *mut u8,
    u16: *mut u16,
}

pub union UPunMut32 {
    u8: *mut u8,
    u16: *mut u16,
    u32: *mut u32,
}

pub union UPunMut64 {
    u8: *mut u8,
    u16: *mut u16,
    u32: *mut u32,
    u64: *mut u64,
}

pub union UPun1Mut28 {
    u8: *mut u8,
    u16: *mut u16,
    u32: *mut u32,
    u64: *mut u64,
    u128: *mut u128,
}
