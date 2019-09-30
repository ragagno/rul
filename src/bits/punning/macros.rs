#[macro_export]
macro_rules! implement_constant {
    ($trait:ty, $union:ty, $type:tt) => {
        implement_constant!($trait, $union, $type, $type);
    };

    ($trait:ty, $union:ty, $member:ident, $type:ty) => {
        impl $trait for $union {
            #[inline]
            unsafe fn get(&self) -> *const $type {
                return self.$member;
            }
        }
    };
}

#[macro_export]
macro_rules! implement_mutable {
    ($trait:ty, $union:ty, $type:tt) => {
        implement_mutable!($trait, $union, $type, $type);
    };

    ($trait:ty, $union:ty, $member:ident, $type:ty) => {
        impl $trait for $union {
            #[inline]
            unsafe fn get(&self) -> *mut $type {
                return self.$member;
            }
        }
    };
}
