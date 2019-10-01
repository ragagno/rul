pub enum Enum {
    Ipv4(u32),
    Ipv6(u128),
}

pub trait Get {
    fn get(&self) -> Enum;
}

pub trait Set: Get {
    fn set(&mut self, value: Enum);
}

#[macro_export]
macro_rules! rul_implement_address_get {
    ($type:ty, $member:ident) => {
        impl $crate::interface::address::Get for $type {
            #[inline]
            fn get(&self) -> $crate::interface::address::Enum {
                return self.$member;
            }
        }
    };
}

#[macro_export]
macro_rules! rul_implement_address_set {
    ($type:ty, $member:ident) => {
        impl $crate::interface::address::Set for $type {
            #[inline]
            fn set(&mut self, value: $crate::interface::address::Enum) {
                self.$member = value;
            }
        }
    };
}
