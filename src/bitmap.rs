use scrypto::prelude::*;

#[blueprint]
mod bitmap {
    struct TickBitmap {
        tickBitmap: HashMap<i16, u128>,
    }

    impl TickBitmap {
        fn position(tick: i32) -> (i32, u8) {
            let word_pos = tick >> 8;
            let bit_pos = (tick % 256) as u8;
            (word_pos, bit_pos)
        }
    }
}
