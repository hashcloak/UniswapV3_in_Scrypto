pub mod math {
    use std::{
        arch::asm,
        ops::{Add, Div},
    };

    pub fn calc_amount0_delta(
        mut sqrt_price_ax96: u128,
        mut sqrt_price_bx96: u128,
        liquidity: u128,
    ) {
        if sqrt_price_ax96 > sqrt_price_bx96 {
            (sqrt_price_ax96, sqrt_price_bx96) = (sqrt_price_bx96, sqrt_price_ax96);
        }

        assert!(sqrt_price_ax96 > 0);

        let amount0 = div_rounding_up(
            mulDivRoundingUp(
                // liquidity << RESOLUTION,
                sqrt_price_bx96 - sqrt_price_ax96,
                sqrt_price_bx96,
            ),
            sqrtPriceAX96,
        );
    }

    pub fn div_rounding_up(numerator: u128, denominator: u128) -> u128 {
        let quotient = numerator / denominator;
        let remainder = numerator % denominator;

        if remainder > 0 {
            quotient + 1
        } else {
            quotient
        }
    }

    pub fn mulDivRoundingUp(a: u128, b: u128, denominator: u128) {
        // let mut result = mulDiv(a, b, denominator);
        if mulmod(a, b, denominator) > 0 {
            require(result < u128::MAX);
            result += 1;
        }
    }
}
