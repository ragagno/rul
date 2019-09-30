pub trait Constant: super::u64::Constant {
    unsafe fn get(&self) -> *const u128;
}

pub trait Mutable: Constant + super::u64::Mutable {
    unsafe fn get(&self) -> *mut u128;
}

#[repr(C)]
pub union Const {
    pub u8: *const u8,
    pub u16: *const u16,
    pub u32: *const u32,
    pub u64: *const u64,
    pub u128: *const u128,
}

#[repr(C)]
pub union Mut {
    pub u8: *mut u8,
    pub u16: *mut u16,
    pub u32: *mut u32,
    pub u64: *mut u64,
    pub u128: *mut u128,
}

implement_constant!(super::u8::Constant, Const, u8);
implement_constant!(super::u16::Constant, Const, u16);
implement_constant!(super::u32::Constant, Const, u32);
implement_constant!(super::u64::Constant, Const, u64);
implement_constant!(super::u128::Constant, Const, u128);

implement_constant!(super::u8::Constant, Mut, u8);
implement_constant!(super::u16::Constant, Mut, u16);
implement_constant!(super::u32::Constant, Mut, u32);
implement_constant!(super::u64::Constant, Mut, u64);
implement_constant!(super::u128::Constant, Mut, u128);
implement_mutable!(super::u8::Mutable, Mut, u8);
implement_mutable!(super::u16::Mutable, Mut, u16);
implement_mutable!(super::u32::Mutable, Mut, u32);
implement_mutable!(super::u64::Mutable, Mut, u64);
implement_mutable!(super::u128::Mutable, Mut, u128);
