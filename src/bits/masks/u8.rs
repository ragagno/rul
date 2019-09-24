macro_rules! define_bits {
    ({$name:ident, $iota:literal}) => { pub const $name: u8 = 1u8 << $iota; };

    ($({$names:ident, $iotas:literal}),+) => { $(define_bits!({$names, $iotas});)+ };
}

macro_rules! define_bits_lsd {
    ({$name:ident, $iota:literal}) => { pub const $name: u8 = !0u8 >> $iota; };

    ($({$names:ident, $iotas:literal}),+) => { $(define_bits_lsd!({$names, $iotas});)+ };
}

macro_rules! define_bits_msd {
    ({$name:ident, $iota:literal}) => { pub const $name: u8 = !0u8 << $iota; };

    ($({$names:ident, $iotas:literal}),+) => { $(define_bits_msd!({$names, $iotas});)+ };
}

define_bits!({BIT_0, 0u8}, {BIT_1, 1u8}, {BIT_2, 2u8}, {BIT_3, 3u8}, {BIT_4, 4u8}, {BIT_5, 5u8}, {BIT_6, 6u8}, {BIT_7, 7u8});

define_bits_lsd!({BIT_0_LSD, 7u8}, {BIT_1_LSD, 6u8}, {BIT_2_LSD, 5u8}, {BIT_3_LSD, 4u8}, {BIT_4_LSD, 3u8}, {BIT_5_LSD, 2u8}, {BIT_6_LSD, 1u8}, {BIT_7_LSD, 0u8});

define_bits_msd!({BIT_0_MSD, 0u8}, {BIT_1_MSD, 1u8}, {BIT_2_MSD, 2u8}, {BIT_3_MSD, 3u8}, {BIT_4_MSD, 4u8}, {BIT_5_MSD, 5u8}, {BIT_6_MSD, 6u8}, {BIT_7_MSD, 7u8});

#[cfg(test)]
mod tests {
    use super::*;

    mod assertions {
        #[test]
        fn bits() {
            assert_eq!(0b00000001u8, super::BIT_0);
            assert_eq!(0b00000010u8, super::BIT_1);
            assert_eq!(0b00000100u8, super::BIT_2);
            assert_eq!(0b00001000u8, super::BIT_3);
            assert_eq!(0b00010000u8, super::BIT_4);
            assert_eq!(0b00100000u8, super::BIT_5);
            assert_eq!(0b01000000u8, super::BIT_6);
            assert_eq!(0b10000000u8, super::BIT_7);
        }

        #[test]
        fn bits_lsd() {
            assert_eq!(0b00000001u8, super::BIT_0_LSD);
            assert_eq!(0b00000011u8, super::BIT_1_LSD);
            assert_eq!(0b00000111u8, super::BIT_2_LSD);
            assert_eq!(0b00001111u8, super::BIT_3_LSD);
            assert_eq!(0b00011111u8, super::BIT_4_LSD);
            assert_eq!(0b00111111u8, super::BIT_5_LSD);
            assert_eq!(0b01111111u8, super::BIT_6_LSD);
            assert_eq!(0b11111111u8, super::BIT_7_LSD);
        }

        #[test]
        fn bits_msd() {
            assert_eq!(0b11111111u8, super::BIT_0_MSD);
            assert_eq!(0b11111110u8, super::BIT_1_MSD);
            assert_eq!(0b11111100u8, super::BIT_2_MSD);
            assert_eq!(0b11111000u8, super::BIT_3_MSD);
            assert_eq!(0b11110000u8, super::BIT_4_MSD);
            assert_eq!(0b11100000u8, super::BIT_5_MSD);
            assert_eq!(0b11000000u8, super::BIT_6_MSD);
            assert_eq!(0b10000000u8, super::BIT_7_MSD);
        }
    }
}
