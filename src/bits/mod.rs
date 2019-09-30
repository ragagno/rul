pub mod masks;
pub mod endian;

const BITS_PER_BYTE: usize = 8usize;

const BITS_PER_BYTE_U8: u8 = 8u8;
const BITS_PER_BYTE_U16: u16 = 8u16;
const BITS_PER_BYTE_U32: u32 = 8u32;
const BITS_PER_BYTE_U64: u64 = 8u64;
const BITS_PER_BYTE_U128: u128 = 8u128;

macro_rules! implement_const {
    ($name:ident, $({$from_name:ident, $get_name:ident, $type:tt}),+) => {
        implement_const!($name, $({$from_name, $get_name, $type, $type}),+);
    };

    ($name:ident, $({$from_name:ident, $get_name:ident, $member_name:ident, $type:ty}),+) => {
        pub union $name {
            $($member_name: *const $type),+
        }

        impl $name {
            $(
                #[inline]
                pub unsafe fn $from_name(raw: *const $type) -> Self {
                    return Self { $member_name: raw };
                }
            )+

            $(
                #[inline]
                pub fn $get_name(&self) -> $type {
                    unsafe {
                        return *self.$member_name
                    }
                }
            )+
        }
    };
}

macro_rules! implement_mut {
    ($name:ident, $({$from_name:ident, $get_name:ident, $set_name:ident, $type:tt}),+) => {
        implement_mut!($name, $({$from_name, $get_name, $set_name, $type, $type}),+);
    };

    ($name:ident, $({$from_name:ident, $get_name:ident, $set_name:ident, $member_name:ident, $type:ty}),+) => {
        pub union $name {
            $($member_name: *mut $type),+
        }

        impl $name {
            $(
                #[inline]
                pub unsafe fn $from_name(raw: *mut $type) -> Self {
                    return Self { $member_name: raw };
                }
            )+

            $(
                #[inline]
                pub fn $get_name(&self) -> $type {
                    unsafe {
                        return *self.$member_name
                    }
                }
            )+

            $(
                #[inline]
                pub fn $set_name(&mut self, value: $type) {
                    unsafe {
                        *self.$member_name = value;
                    }
                }
            )+
        }
    };
}

implement_const!(UPunConst8, {from_u8, get_u8, u8});
implement_const!(UPunConst16, {from_u8, get_u8, u8}, {from_u16, get_u16, u16});
implement_const!(UPunConst32, {from_u8, get_u8, u8}, {from_u16, get_u16, u16}, {from_u32, get_u32, u32});
implement_const!(UPunConst64, {from_u8, get_u8, u8}, {from_u16, get_u16, u16}, {from_u32, get_u32, u32}, {from_u64, get_u64, u64});
implement_const!(UPunConst128, {from_u8, get_u8, u8}, {from_u16, get_u16, u16}, {from_u32, get_u32, u32}, {from_u64, get_u64, u64}, {from_u128, get_u128, u128});

implement_mut!(UPunMut8, {from_u8, get_u8, set_u8, u8});
implement_mut!(UPunMut16, {from_u8, get_u8, set_u8, u8}, {from_u16, get_u16, set_u16, u16});
implement_mut!(UPunMut32, {from_u8, get_u8, set_u8, u8}, {from_u16, get_u16, set_u16, u16}, {from_u32, get_u32, set_u32, u32});
implement_mut!(UPunMut64, {from_u8, get_u8, set_u8, u8}, {from_u16, get_u16, set_u16, u16}, {from_u32, get_u32, set_u32, u32}, {from_u64, get_u64, set_u64, u64});
implement_mut!(UPunMut128, {from_u8, get_u8, set_u8, u8}, {from_u16, get_u16, set_u16, u16}, {from_u32, get_u32, set_u32, u32}, {from_u64, get_u64, set_u64, u64}, {from_u128, get_u128, set_u128, u128});
