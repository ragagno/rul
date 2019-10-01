pub trait Get {
    fn get(&self) -> u16;
}

pub trait Set: Get {
    fn set(&mut self, value: u16);
}

#[macro_export]
macro_rules! rul_implement_port_get {
    ($type:ty, $member:ident) => {
        impl $crate::interface::port::Get for $type {
            #[inline]
            fn get(&self) -> u16 {
                return self.$member;
            }
        }
    };
}

#[macro_export]
macro_rules! rul_implement_port_set {
    ($type:ty, $member:ident) => {
        impl $crate::interface::port::Set for $type {
            #[inline]
            fn set(&mut self, value: u16) {
                self.$member = value;
            }
        }
    };
}
