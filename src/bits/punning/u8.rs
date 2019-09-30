pub trait Constant {
    unsafe fn get(&self) -> *const u8;
}

pub trait Mutable: Constant {
    unsafe fn get(&self) -> *mut u8;
}

#[repr(C)]
pub union Const {
    pub u8: *const u8,
}

#[repr(C)]
pub union Mut {
    pub u8: *mut u8,
}

implement_constant!(super::u8::Constant, Const, u8);

implement_constant!(super::u8::Constant, Mut, u8);
implement_mutable!(super::u8::Mutable, Mut, u8);
