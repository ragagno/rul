pub mod masks;

pub union UPun8 {
    u8: * u8,
}

pub union UPun16 {
    u8: * u8,
    u16: * u16,
}

pub union UPun32 {
    u8: * u8,
    u16: * u16,
    u32: * u32,
}

pub union UPun64 {
    u8: * u8,
    u16: * u16,
    u32: * u32,
    u64: * u64,
}

pub union UPun128 {
    u8: * u8,
    u16: * u16,
    u32: * u32,
    u64: * u64,
    u128: * u128,
}
