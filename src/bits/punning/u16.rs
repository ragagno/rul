pub trait Constant: super::u8::Constant {
    unsafe fn get(&self) -> *const u16;
}

pub trait Mutable: Constant + super::u8::Mutable {
    unsafe fn get(&self) -> *mut u16;
}

#[repr(C)]
pub union Const {
    pub u8: *const u8,
    pub u16: *const u16,
}

#[repr(C)]
pub union Mut {
    pub u8: *mut u8,
    pub u16: *mut u16,
}

implement_constant!(super::u8::Constant, Const, u8);
implement_constant!(super::u16::Constant, Const, u16);

implement_constant!(super::u8::Constant, Mut, u8);
implement_constant!(super::u16::Constant, Mut, u16);
implement_mutable!(super::u8::Mutable, Mut, u8);
implement_mutable!(super::u16::Mutable, Mut, u16);
