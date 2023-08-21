use scrypto::prelude::*;

#[blueprint]
mod bitmap {
    struct TickBitmap {
        tick_bitmap: HashMap<i16, u128>,
    }

    impl TickBitmap {
        pub fn instantiate_bitmap() -> Global<TickBitmap> {
            Self {
                tick_bitmap: HashMap::new(),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }

        fn position(tick: i32) -> (i16, u8) {
            let word_pos = (tick >> 8) as i16;
            let bit_pos = (tick % 256) as u8;
            (word_pos, bit_pos)
        }

        fn flipTick(&mut self, tick: i32, tick_spacing: i32) {
            assert!((tick % tick_spacing) == 0);
            let (word_pos, bit_pos) = Self::position(tick / tick_spacing);
            let mask: u128 = 1 << bit_pos;

            match self.tick_bitmap.get_mut(&word_pos) {
                Some(mut tick_bitmap) => {
                    let mut bitmap_value = *tick_bitmap ^ mask;
                    tick_bitmap = &mut bitmap_value;
                }

                None => {
                    self.tick_bitmap.insert(word_pos, mask);
                }
            }
        }

        fn nextInitializedTickWithinOneWord(
            &mut self,
            tick: i32,
            tick_spacing: i32,
            lte: bool,
        ) -> (i32, bool) {
            let mut compressed = tick / tick_spacing;
            if tick < 0 && tick % tick_spacing != 0 {
                compressed -= 1;
            }

            let initialized;
            let next;

            if lte {
                let (word_pos, bit_pos) = Self::position(compressed);
                let mask = BnumU256::from(((1 << bit_pos) - 1 + (1 << bit_pos)) as u128);

                let masked: BnumU256;

                match self.tick_bitmap.get(&word_pos) {
                    Some(tick_bitmap) => {
                        masked = BnumU256::from(*tick_bitmap) & mask;
                    }

                    None => {
                        panic!("Not able to retrieve tick Bitmap");
                    }
                }

                initialized = masked != BnumU256::from(0u128);

                next = if initialized {
                    (compressed - (bit_pos - Self::most_significant_bit(masked)) as i32)
                        * tick_spacing
                } else {
                    (compressed - (bit_pos as i32)) * tick_spacing
                };
            } else {
                let (word_pos, bit_pos) = Self::position(compressed + 1);

                let mask: BnumU256 = BnumU256::from(!((1 << bit_pos) - 1) as u128);
                // uint256 masked = self[wordPos] & mask;
                let masked: BnumU256;

                match self.tick_bitmap.get(&word_pos) {
                    Some(tick_bitmap) => {
                        masked = BnumU256::from(*tick_bitmap) & mask;
                    }

                    None => {
                        panic!("Not able to retrieve tick Bitmap");
                    }
                }

                // if there are no initialized ticks to the left of the current tick, return leftmost in the word
                initialized = masked != BnumU256::from(0u128);

                next = if initialized {
                    (compressed + 1 + (Self::least_significant_bit(masked) - bit_pos) as i32)
                        * tick_spacing
                } else {
                    (compressed + 1 + (std::u8::MAX - bit_pos) as i32) * tick_spacing
                };

                // initialized
                //     ? (compressed + 1 + int24(uint24((BitMath.leastSignificantBit(masked) - bitPos)))) * tickSpacing
                //     : (compressed + 1 + int24(uint24((type(uint8).max - bitPos)))) * tickSpacing;
            }

            (next, initialized)
        }

        fn most_significant_bit(mut x: BnumU256) -> u8 {
            assert!(x > BnumU256::from(0u128));

            let mut r: u8 = 0;

            if x > BnumU256::from(340282366920938463463374607431768211455u128) {
                x >>= BnumU256::from(128u128);
                r += 128;
            }
            if x >= BnumU256::from(0x10000000000000000u128) {
                x >>= BnumU256::from(64u128);
                r += 64;
            }
            if x >= BnumU256::from(0x100000000u128) {
                x >>= BnumU256::from(32u128);
                r += 32;
            }
            if x >= BnumU256::from(0x10000u128) {
                x >>= BnumU256::from(16u128);
                r += 16;
            }
            if x >= BnumU256::from(0x100u128) {
                x >>= BnumU256::from(8u128);
                r += 8;
            }
            if x >= BnumU256::from(0x10u128) {
                x >>= BnumU256::from(4u128);
                r += 4;
            }
            if x >= BnumU256::from(0x4u128) {
                x >>= BnumU256::from(2u128);
                r += 2;
            }
            if x >= BnumU256::from(0x2u128) {
                r += 1;
            }

            return r;
        }

        fn least_significant_bit(mut x: BnumU256) -> u8 {
            assert!(x > BnumU256::from(0u128));

            let mut r: u8 = 255;

            if x & BnumU256::from(std::u128::MAX) > BnumU256::from(0u128) {
                r -= 128;
            } else {
                x >>= BnumU256::from(128u128);
            }

            if x & BnumU256::from(std::u64::MAX) > BnumU256::from(0u128) {
                r -= 64;
            } else {
                x >>= BnumU256::from(64u128);
            }

            if x & BnumU256::from(std::u32::MAX) > BnumU256::from(0u128) {
                r -= 32;
            } else {
                x >>= BnumU256::from(32u128);
            }

            if x & BnumU256::from(std::u16::MAX) > BnumU256::from(0u128) {
                r -= 16;
            } else {
                x >>= BnumU256::from(16u128);
            }

            if x & BnumU256::from(std::u8::MAX) > BnumU256::from(0u128) {
                r -= 8;
            } else {
                x >>= BnumU256::from(8u128);
            }

            if x & BnumU256::from(0xfu128) > BnumU256::from(0u128) {
                r -= 4;
            } else {
                x >>= BnumU256::from(4u128);
            }

            if x & BnumU256::from(0x3u128) > BnumU256::from(0u128) {
                r -= 2;
            } else {
                x >>= BnumU256::from(2u128);
            }

            if x & BnumU256::from(0x1u128) > BnumU256::from(0u128) {
                r -= 1;
            }

            return r;
        }
    }
}
