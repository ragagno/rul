macro_rules! define_bits {
    ({$name:ident, $iota:literal}) => { pub const $name: u32 = 1u32 << $iota; };

    ($({$names:ident, $iotas:literal}),+) => { $(define_bits!({$names, $iotas});)+ };
}

macro_rules! define_bits_lsd {
    ({$name:ident, $iota:literal}) => { pub const $name: u32 = !0u32 >> $iota; };

    ($({$names:ident, $iotas:literal}),+) => { $(define_bits_lsd!({$names, $iotas});)+ };
}

macro_rules! define_bits_msd {
    ({$name:ident, $iota:literal}) => { pub const $name: u32 = !0u32 << $iota; };

    ($({$names:ident, $iotas:literal}),+) => { $(define_bits_msd!({$names, $iotas});)+ };
}

macro_rules! define_bytes {
    ({$name:ident, $iota:literal}) => { pub const $name: u32 = 0xFFu32 << 8u32 * $iota; };

    ($({$names:ident, $iotas:literal}),+) => { $(define_bytes!({$names, $iotas});)+ };
}

macro_rules! define_bytes_lsd {
    ({$name:ident, $iota:literal}) => { pub const $name: u32 = !0u32 >> 8u32 * $iota; };

    ($({$names:ident, $iotas:literal}),+) => { $(define_bytes_lsd!({$names, $iotas});)+ };
}

macro_rules! define_bytes_msd {
    ({$name:ident, $iota:literal}) => { pub const $name: u32 = !0u32 << 8u32 * $iota; };

    ($({$names:ident, $iotas:literal}),+) => { $(define_bytes_msd!({$names, $iotas});)+ };
}

define_bits!({BIT_0, 0u32}, {BIT_1, 1u32}, {BIT_2, 2u32}, {BIT_3, 3u32}, {BIT_4, 4u32}, {BIT_5, 5u32}, {BIT_6, 6u32}, {BIT_7, 7u32}, {BIT_8, 8u32}, {BIT_9, 9u32}, {BIT_10, 10u32}, {BIT_11, 11u32}, {BIT_12, 12u32}, {BIT_13, 13u32}, {BIT_14, 14u32}, {BIT_15, 15u32}, {BIT_16, 16u32}, {BIT_17, 17u32}, {BIT_18, 18u32}, {BIT_19, 19u32}, {BIT_20, 20u32}, {BIT_21, 21u32}, {BIT_22, 22u32}, {BIT_23, 23u32}, {BIT_24, 24u32}, {BIT_25, 25u32}, {BIT_26, 26u32}, {BIT_27, 27u32}, {BIT_28, 28u32}, {BIT_29, 29u32}, {BIT_30, 30u32}, {BIT_31, 31u32});

define_bits_lsd!({BIT_0_LSD, 31u32}, {BIT_1_LSD, 30u32}, {BIT_2_LSD, 29u32}, {BIT_3_LSD, 28u32}, {BIT_4_LSD, 27u32}, {BIT_5_LSD, 26u32}, {BIT_6_LSD, 25u32}, {BIT_7_LSD, 24u32}, {BIT_8_LSD, 23u32}, {BIT_9_LSD, 22u32}, {BIT_10_LSD, 21u32}, {BIT_11_LSD, 20u32}, {BIT_12_LSD, 19u32}, {BIT_13_LSD, 18u32}, {BIT_14_LSD, 17u32}, {BIT_15_LSD, 16u32}, {BIT_16_LSD, 15u32}, {BIT_17_LSD, 14u32}, {BIT_18_LSD, 13u32}, {BIT_19_LSD, 12u32}, {BIT_20_LSD, 11u32}, {BIT_21_LSD, 10u32}, {BIT_22_LSD, 9u32}, {BIT_23_LSD, 8u32}, {BIT_24_LSD, 7u32}, {BIT_25_LSD, 6u32}, {BIT_26_LSD, 5u32}, {BIT_27_LSD, 4u32}, {BIT_28_LSD, 3u32}, {BIT_29_LSD, 2u32}, {BIT_30_LSD, 1u32}, {BIT_31_LSD, 0u32});

define_bits_msd!({BIT_0_MSD, 0u32}, {BIT_1_MSD, 1u32}, {BIT_2_MSD, 2u32}, {BIT_3_MSD, 3u32}, {BIT_4_MSD, 4u32}, {BIT_5_MSD, 5u32}, {BIT_6_MSD, 6u32}, {BIT_7_MSD, 7u32}, {BIT_8_MSD, 8u32}, {BIT_9_MSD, 9u32}, {BIT_10_MSD, 10u32}, {BIT_11_MSD, 11u32}, {BIT_12_MSD, 12u32}, {BIT_13_MSD, 13u32}, {BIT_14_MSD, 14u32}, {BIT_15_MSD, 15u32}, {BIT_16_MSD, 16u32}, {BIT_17_MSD, 17u32}, {BIT_18_MSD, 18u32}, {BIT_19_MSD, 19u32}, {BIT_20_MSD, 20u32}, {BIT_21_MSD, 21u32}, {BIT_22_MSD, 22u32}, {BIT_23_MSD, 23u32}, {BIT_24_MSD, 24u32}, {BIT_25_MSD, 25u32}, {BIT_26_MSD, 26u32}, {BIT_27_MSD, 27u32}, {BIT_28_MSD, 28u32}, {BIT_29_MSD, 29u32}, {BIT_30_MSD, 30u32}, {BIT_31_MSD, 31u32});

define_bytes!({BYTE_0, 0u32}, {BYTE_1, 1u32}, {BYTE_2, 2u32}, {BYTE_3, 3u32});

define_bytes_lsd!({BYTE_0_LSD, 3u32}, {BYTE_1_LSD, 2u32}, {BYTE_2_LSD, 1u32}, {BYTE_3_LSD, 0u32});

define_bytes_msd!({BYTE_0_MSD, 0u32}, {BYTE_1_MSD, 1u32}, {BYTE_2_MSD, 2u32}, {BYTE_3_MSD, 3u32});

#[cfg(test)]
mod tests {
    use super::*;

    mod assertions {
        #[test]
        fn bits() {
            assert_eq!(0b00000000_00000000_00000000_00000001u32, super::BIT_0);
            assert_eq!(0b00000000_00000000_00000000_00000010u32, super::BIT_1);
            assert_eq!(0b00000000_00000000_00000000_00000100u32, super::BIT_2);
            assert_eq!(0b00000000_00000000_00000000_00001000u32, super::BIT_3);
            assert_eq!(0b00000000_00000000_00000000_00010000u32, super::BIT_4);
            assert_eq!(0b00000000_00000000_00000000_00100000u32, super::BIT_5);
            assert_eq!(0b00000000_00000000_00000000_01000000u32, super::BIT_6);
            assert_eq!(0b00000000_00000000_00000000_10000000u32, super::BIT_7);
            assert_eq!(0b00000000_00000000_00000001_00000000u32, super::BIT_8);
            assert_eq!(0b00000000_00000000_00000010_00000000u32, super::BIT_9);
            assert_eq!(0b00000000_00000000_00000100_00000000u32, super::BIT_10);
            assert_eq!(0b00000000_00000000_00001000_00000000u32, super::BIT_11);
            assert_eq!(0b00000000_00000000_00010000_00000000u32, super::BIT_12);
            assert_eq!(0b00000000_00000000_00100000_00000000u32, super::BIT_13);
            assert_eq!(0b00000000_00000000_01000000_00000000u32, super::BIT_14);
            assert_eq!(0b00000000_00000000_10000000_00000000u32, super::BIT_15);
            assert_eq!(0b00000000_00000001_00000000_00000000u32, super::BIT_16);
            assert_eq!(0b00000000_00000010_00000000_00000000u32, super::BIT_17);
            assert_eq!(0b00000000_00000100_00000000_00000000u32, super::BIT_18);
            assert_eq!(0b00000000_00001000_00000000_00000000u32, super::BIT_19);
            assert_eq!(0b00000000_00010000_00000000_00000000u32, super::BIT_20);
            assert_eq!(0b00000000_00100000_00000000_00000000u32, super::BIT_21);
            assert_eq!(0b00000000_01000000_00000000_00000000u32, super::BIT_22);
            assert_eq!(0b00000000_10000000_00000000_00000000u32, super::BIT_23);
            assert_eq!(0b00000001_00000000_00000000_00000000u32, super::BIT_24);
            assert_eq!(0b00000010_00000000_00000000_00000000u32, super::BIT_25);
            assert_eq!(0b00000100_00000000_00000000_00000000u32, super::BIT_26);
            assert_eq!(0b00001000_00000000_00000000_00000000u32, super::BIT_27);
            assert_eq!(0b00010000_00000000_00000000_00000000u32, super::BIT_28);
            assert_eq!(0b00100000_00000000_00000000_00000000u32, super::BIT_29);
            assert_eq!(0b01000000_00000000_00000000_00000000u32, super::BIT_30);
            assert_eq!(0b10000000_00000000_00000000_00000000u32, super::BIT_31);
        }

        #[test]
        fn bits_lsd() {
            assert_eq!(0b00000000_00000000_00000000_00000001u32, super::BIT_0_LSD);
            assert_eq!(0b00000000_00000000_00000000_00000011u32, super::BIT_1_LSD);
            assert_eq!(0b00000000_00000000_00000000_00000111u32, super::BIT_2_LSD);
            assert_eq!(0b00000000_00000000_00000000_00001111u32, super::BIT_3_LSD);
            assert_eq!(0b00000000_00000000_00000000_00011111u32, super::BIT_4_LSD);
            assert_eq!(0b00000000_00000000_00000000_00111111u32, super::BIT_5_LSD);
            assert_eq!(0b00000000_00000000_00000000_01111111u32, super::BIT_6_LSD);
            assert_eq!(0b00000000_00000000_00000000_11111111u32, super::BIT_7_LSD);
            assert_eq!(0b00000000_00000000_00000001_11111111u32, super::BIT_8_LSD);
            assert_eq!(0b00000000_00000000_00000011_11111111u32, super::BIT_9_LSD);
            assert_eq!(0b00000000_00000000_00000111_11111111u32, super::BIT_10_LSD);
            assert_eq!(0b00000000_00000000_00001111_11111111u32, super::BIT_11_LSD);
            assert_eq!(0b00000000_00000000_00011111_11111111u32, super::BIT_12_LSD);
            assert_eq!(0b00000000_00000000_00111111_11111111u32, super::BIT_13_LSD);
            assert_eq!(0b00000000_00000000_01111111_11111111u32, super::BIT_14_LSD);
            assert_eq!(0b00000000_00000000_11111111_11111111u32, super::BIT_15_LSD);
            assert_eq!(0b00000000_00000001_11111111_11111111u32, super::BIT_16_LSD);
            assert_eq!(0b00000000_00000011_11111111_11111111u32, super::BIT_17_LSD);
            assert_eq!(0b00000000_00000111_11111111_11111111u32, super::BIT_18_LSD);
            assert_eq!(0b00000000_00001111_11111111_11111111u32, super::BIT_19_LSD);
            assert_eq!(0b00000000_00011111_11111111_11111111u32, super::BIT_20_LSD);
            assert_eq!(0b00000000_00111111_11111111_11111111u32, super::BIT_21_LSD);
            assert_eq!(0b00000000_01111111_11111111_11111111u32, super::BIT_22_LSD);
            assert_eq!(0b00000000_11111111_11111111_11111111u32, super::BIT_23_LSD);
            assert_eq!(0b00000001_11111111_11111111_11111111u32, super::BIT_24_LSD);
            assert_eq!(0b00000011_11111111_11111111_11111111u32, super::BIT_25_LSD);
            assert_eq!(0b00000111_11111111_11111111_11111111u32, super::BIT_26_LSD);
            assert_eq!(0b00001111_11111111_11111111_11111111u32, super::BIT_27_LSD);
            assert_eq!(0b00011111_11111111_11111111_11111111u32, super::BIT_28_LSD);
            assert_eq!(0b00111111_11111111_11111111_11111111u32, super::BIT_29_LSD);
            assert_eq!(0b01111111_11111111_11111111_11111111u32, super::BIT_30_LSD);
            assert_eq!(0b11111111_11111111_11111111_11111111u32, super::BIT_31_LSD);
        }

        #[test]
        fn bits_msd() {
            assert_eq!(0b11111111_11111111_11111111_11111111u32, super::BIT_0_MSD);
            assert_eq!(0b11111111_11111111_11111111_11111110u32, super::BIT_1_MSD);
            assert_eq!(0b11111111_11111111_11111111_11111100u32, super::BIT_2_MSD);
            assert_eq!(0b11111111_11111111_11111111_11111000u32, super::BIT_3_MSD);
            assert_eq!(0b11111111_11111111_11111111_11110000u32, super::BIT_4_MSD);
            assert_eq!(0b11111111_11111111_11111111_11100000u32, super::BIT_5_MSD);
            assert_eq!(0b11111111_11111111_11111111_11000000u32, super::BIT_6_MSD);
            assert_eq!(0b11111111_11111111_11111111_10000000u32, super::BIT_7_MSD);
            assert_eq!(0b11111111_11111111_11111111_00000000u32, super::BIT_8_MSD);
            assert_eq!(0b11111111_11111111_11111110_00000000u32, super::BIT_9_MSD);
            assert_eq!(0b11111111_11111111_11111100_00000000u32, super::BIT_10_MSD);
            assert_eq!(0b11111111_11111111_11111000_00000000u32, super::BIT_11_MSD);
            assert_eq!(0b11111111_11111111_11110000_00000000u32, super::BIT_12_MSD);
            assert_eq!(0b11111111_11111111_11100000_00000000u32, super::BIT_13_MSD);
            assert_eq!(0b11111111_11111111_11000000_00000000u32, super::BIT_14_MSD);
            assert_eq!(0b11111111_11111111_10000000_00000000u32, super::BIT_15_MSD);
            assert_eq!(0b11111111_11111111_00000000_00000000u32, super::BIT_16_MSD);
            assert_eq!(0b11111111_11111110_00000000_00000000u32, super::BIT_17_MSD);
            assert_eq!(0b11111111_11111100_00000000_00000000u32, super::BIT_18_MSD);
            assert_eq!(0b11111111_11111000_00000000_00000000u32, super::BIT_19_MSD);
            assert_eq!(0b11111111_11110000_00000000_00000000u32, super::BIT_20_MSD);
            assert_eq!(0b11111111_11100000_00000000_00000000u32, super::BIT_21_MSD);
            assert_eq!(0b11111111_11000000_00000000_00000000u32, super::BIT_22_MSD);
            assert_eq!(0b11111111_10000000_00000000_00000000u32, super::BIT_23_MSD);
            assert_eq!(0b11111111_00000000_00000000_00000000u32, super::BIT_24_MSD);
            assert_eq!(0b11111110_00000000_00000000_00000000u32, super::BIT_25_MSD);
            assert_eq!(0b11111100_00000000_00000000_00000000u32, super::BIT_26_MSD);
            assert_eq!(0b11111000_00000000_00000000_00000000u32, super::BIT_27_MSD);
            assert_eq!(0b11110000_00000000_00000000_00000000u32, super::BIT_28_MSD);
            assert_eq!(0b11100000_00000000_00000000_00000000u32, super::BIT_29_MSD);
            assert_eq!(0b11000000_00000000_00000000_00000000u32, super::BIT_30_MSD);
            assert_eq!(0b10000000_00000000_00000000_00000000u32, super::BIT_31_MSD);
        }

        #[test]
        fn bytes() {
            assert_eq!(0x00_00_00_FFu32, super::BYTE_0);
            assert_eq!(0x00_00_FF_00u32, super::BYTE_1);
            assert_eq!(0x00_FF_00_00u32, super::BYTE_2);
            assert_eq!(0xFF_00_00_00u32, super::BYTE_3);
        }

        #[test]
        fn bytes_lsd() {
            assert_eq!(0x00_00_00_FFu32, super::BYTE_0_LSD);
            assert_eq!(0x00_00_FF_FFu32, super::BYTE_1_LSD);
            assert_eq!(0x00_FF_FF_FFu32, super::BYTE_2_LSD);
            assert_eq!(0xFF_FF_FF_FFu32, super::BYTE_3_LSD);
        }

        #[test]
        fn bytes_msd() {
            assert_eq!(0xFF_FF_FF_FFu32, super::BYTE_0_MSD);
            assert_eq!(0xFF_FF_FF_00u32, super::BYTE_1_MSD);
            assert_eq!(0xFF_FF_00_00u32, super::BYTE_2_MSD);
            assert_eq!(0xFF_00_00_00u32, super::BYTE_3_MSD);
        }
    }
}
