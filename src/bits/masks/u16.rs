macro_rules! define_bits {
    ({$name:ident, $iota:literal}) => { pub const $name: u16 = 1u16 << $iota; };

    ($({$names:ident, $iotas:literal}),+) => { $(define_bits!({$names, $iotas});)+ };
}

macro_rules! define_bits_lsd {
    ({$name:ident, $iota:literal}) => { pub const $name: u16 = !0u16 >> $iota; };

    ($({$names:ident, $iotas:literal}),+) => { $(define_bits_lsd!({$names, $iotas});)+ };
}

macro_rules! define_bits_msd {
    ({$name:ident, $iota:literal}) => { pub const $name: u16 = !0u16 << $iota; };

    ($({$names:ident, $iotas:literal}),+) => { $(define_bits_msd!({$names, $iotas});)+ };
}

macro_rules! define_bytes {
    ({$name:ident, $iota:literal}) => { pub const $name: u16 = 0xFFu16 << 8u16 * $iota; };

    ($({$names:ident, $iotas:literal}),+) => { $(define_bytes!({$names, $iotas});)+ };
}

macro_rules! define_bytes_lsd {
    ({$name:ident, $iota:literal}) => { pub const $name: u16 = !0u16 >> 8u16 * $iota; };

    ($({$names:ident, $iotas:literal}),+) => { $(define_bytes_lsd!({$names, $iotas});)+ };
}

macro_rules! define_bytes_msd {
    ({$name:ident, $iota:literal}) => { pub const $name: u16 = !0u16 << 8u16 * $iota; };

    ($({$names:ident, $iotas:literal}),+) => { $(define_bytes_msd!({$names, $iotas});)+ };
}

define_bits!({BIT_0, 0u16}, {BIT_1, 1u16}, {BIT_2, 2u16}, {BIT_3, 3u16}, {BIT_4, 4u16}, {BIT_5, 5u16}, {BIT_6, 6u16}, {BIT_7, 7u16}, {BIT_8, 8u16}, {BIT_9, 9u16}, {BIT_10, 10u16}, {BIT_11, 11u16}, {BIT_12, 12u16}, {BIT_13, 13u16}, {BIT_14, 14u16}, {BIT_15, 15u16});

define_bits_lsd!({BIT_0_LSD, 15u16}, {BIT_1_LSD, 14u16}, {BIT_2_LSD, 13u16}, {BIT_3_LSD, 12u16}, {BIT_4_LSD, 11u16}, {BIT_5_LSD, 10u16}, {BIT_6_LSD, 9u16}, {BIT_7_LSD, 8u16}, {BIT_8_LSD, 7u16}, {BIT_9_LSD, 6u16}, {BIT_10_LSD, 5u16}, {BIT_11_LSD, 4u16}, {BIT_12_LSD, 3u16}, {BIT_13_LSD, 2u16}, {BIT_14_LSD, 1u16}, {BIT_15_LSD, 0u16});

define_bits_msd!({BIT_0_MSD, 0u16}, {BIT_1_MSD, 1u16}, {BIT_2_MSD, 2u16}, {BIT_3_MSD, 3u16}, {BIT_4_MSD, 4u16}, {BIT_5_MSD, 5u16}, {BIT_6_MSD, 6u16}, {BIT_7_MSD, 7u16}, {BIT_8_MSD, 8u16}, {BIT_9_MSD, 9u16}, {BIT_10_MSD, 10u16}, {BIT_11_MSD, 11u16}, {BIT_12_MSD, 12u16}, {BIT_13_MSD, 13u16}, {BIT_14_MSD, 14u16}, {BIT_15_MSD, 15u16});

define_bytes!({BYTE_0, 0u16}, {BYTE_1, 1u16});

define_bytes_lsd!({BYTE_0_LSD, 1u16}, {BYTE_1_LSD, 0u16});

define_bytes_msd!({BYTE_0_MSD, 0u16}, {BYTE_1_MSD, 1u16});

#[cfg(test)]
mod tests {
    use super::*;

    mod assertions {
        #[test]
        fn bits() {
            assert_eq!(0b00000000_00000001u16, super::BIT_0);
            assert_eq!(0b00000000_00000010u16, super::BIT_1);
            assert_eq!(0b00000000_00000100u16, super::BIT_2);
            assert_eq!(0b00000000_00001000u16, super::BIT_3);
            assert_eq!(0b00000000_00010000u16, super::BIT_4);
            assert_eq!(0b00000000_00100000u16, super::BIT_5);
            assert_eq!(0b00000000_01000000u16, super::BIT_6);
            assert_eq!(0b00000000_10000000u16, super::BIT_7);
            assert_eq!(0b00000001_00000000u16, super::BIT_8);
            assert_eq!(0b00000010_00000000u16, super::BIT_9);
            assert_eq!(0b00000100_00000000u16, super::BIT_10);
            assert_eq!(0b00001000_00000000u16, super::BIT_11);
            assert_eq!(0b00010000_00000000u16, super::BIT_12);
            assert_eq!(0b00100000_00000000u16, super::BIT_13);
            assert_eq!(0b01000000_00000000u16, super::BIT_14);
            assert_eq!(0b10000000_00000000u16, super::BIT_15);
        }

        #[test]
        fn bits_lsd() {
            assert_eq!(0b00000000_00000001u16, super::BIT_0_LSD);
            assert_eq!(0b00000000_00000011u16, super::BIT_1_LSD);
            assert_eq!(0b00000000_00000111u16, super::BIT_2_LSD);
            assert_eq!(0b00000000_00001111u16, super::BIT_3_LSD);
            assert_eq!(0b00000000_00011111u16, super::BIT_4_LSD);
            assert_eq!(0b00000000_00111111u16, super::BIT_5_LSD);
            assert_eq!(0b00000000_01111111u16, super::BIT_6_LSD);
            assert_eq!(0b00000000_11111111u16, super::BIT_7_LSD);
            assert_eq!(0b00000001_11111111u16, super::BIT_8_LSD);
            assert_eq!(0b00000011_11111111u16, super::BIT_9_LSD);
            assert_eq!(0b00000111_11111111u16, super::BIT_10_LSD);
            assert_eq!(0b00001111_11111111u16, super::BIT_11_LSD);
            assert_eq!(0b00011111_11111111u16, super::BIT_12_LSD);
            assert_eq!(0b00111111_11111111u16, super::BIT_13_LSD);
            assert_eq!(0b01111111_11111111u16, super::BIT_14_LSD);
            assert_eq!(0b11111111_11111111u16, super::BIT_15_LSD);
        }

        #[test]
        fn bits_msd() {
            assert_eq!(0b11111111_11111111u16, super::BIT_0_MSD);
            assert_eq!(0b11111111_11111110u16, super::BIT_1_MSD);
            assert_eq!(0b11111111_11111100u16, super::BIT_2_MSD);
            assert_eq!(0b11111111_11111000u16, super::BIT_3_MSD);
            assert_eq!(0b11111111_11110000u16, super::BIT_4_MSD);
            assert_eq!(0b11111111_11100000u16, super::BIT_5_MSD);
            assert_eq!(0b11111111_11000000u16, super::BIT_6_MSD);
            assert_eq!(0b11111111_10000000u16, super::BIT_7_MSD);
            assert_eq!(0b11111111_00000000u16, super::BIT_8_MSD);
            assert_eq!(0b11111110_00000000u16, super::BIT_9_MSD);
            assert_eq!(0b11111100_00000000u16, super::BIT_10_MSD);
            assert_eq!(0b11111000_00000000u16, super::BIT_11_MSD);
            assert_eq!(0b11110000_00000000u16, super::BIT_12_MSD);
            assert_eq!(0b11100000_00000000u16, super::BIT_13_MSD);
            assert_eq!(0b11000000_00000000u16, super::BIT_14_MSD);
            assert_eq!(0b10000000_00000000u16, super::BIT_15_MSD);
        }

        #[test]
        fn bytes() {
            assert_eq!(0x00_FFu16, super::BYTE_0);
            assert_eq!(0xFF_00u16, super::BYTE_1);
        }

        #[test]
        fn bytes_lsd() {
            assert_eq!(0x00_FFu16, super::BYTE_0_LSD);
            assert_eq!(0xFF_FFu16, super::BYTE_1_LSD);
        }

        #[test]
        fn bytes_msd() {
            assert_eq!(0xFF_FFu16, super::BYTE_0_MSD);
            assert_eq!(0xFF_00u16, super::BYTE_1_MSD);
        }
    }
}
