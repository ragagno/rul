pub union Const {
    pub u8: *const u8,
    pub u16: *const u16,
    pub u32: *const u32,
    pub u64: *const u64,
    pub u128: *const u128,
}

pub union Mut {
    pub u8: *mut u8,
    pub u16: *mut u16,
    pub u32: *mut u32,
    pub u64: *mut u64,
    pub u128: *mut u128,
}