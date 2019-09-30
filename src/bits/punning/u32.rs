pub trait Constant: super::u16::Constant {
    unsafe fn get(&self) -> *const u32;
}

pub trait Mutable: Constant + super::u16::Mutable {
    unsafe fn get(&self) -> *mut u32;
}

#[repr(C)]
pub union Const {
    pub u8: *const u8,
    pub u16: *const u16,
    pub u32: *const u32,
}

#[repr(C)]
pub union Mut {
    pub u8: *mut u8,
    pub u16: *mut u16,
    pub u32: *mut u32,
}

implement_constant!(super::u8::Constant, Const, u8);
implement_constant!(super::u16::Constant, Const, u16);
implement_constant!(super::u32::Constant, Const, u32);

implement_constant!(super::u8::Constant, Mut, u8);
implement_constant!(super::u16::Constant, Mut, u16);
implement_constant!(super::u32::Constant, Mut, u32);
implement_mutable!(super::u8::Mutable, Mut, u8);
implement_mutable!(super::u16::Mutable, Mut, u16);
implement_mutable!(super::u32::Mutable, Mut, u32);
