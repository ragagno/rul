#[macro_export]
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
                    return unsafe {
                        *self.$member_name
                    };
                }
            )+
        }
    };
}

#[macro_export]
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
                    return unsafe {
                        *self.$member_name
                    };
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
