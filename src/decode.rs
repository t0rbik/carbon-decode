//! This module contains the logic to parse and normalize the strategy data from the Graphene protocol.
//!
//! Carbon SDK uses Decimal.js and BigNumber from @ethersproject/bignumber, which are not available in Rust.
//! One can find carbon-sdk's implementation here:
//! https://github.com/bancorprotocol/carbon-sdk/blob/main/src/utils/encoders.ts#L111

use alloy::primitives::utils::format_units;
use alloy::primitives::{Address, U256};
use eyre::{Result, eyre};
use rust_decimal::prelude::*;
use serde::ser::SerializeStruct;
use std::ops::{Div, Mul};

use crate::contract::{Order, Strategy};

#[derive(Clone)]
struct DecodedOrder {
    pub liquidity: String,
    lowest_rate: String,
    highest_rate: String,
    marginal_rate: String,
}

struct DecodedStrategy {
    id: String,
    token0: Address,
    token1: Address,
    order0: DecodedOrder,
    order1: DecodedOrder,
    encoded: String,
}

#[derive(Debug, Clone)]
pub struct ParsedStrategy {
    pub id: String,
    pub base_token: Address,
    pub quote_token: Address,
    pub buy_price_low: String,
    pub buy_price_marginal: String,
    pub buy_price_high: String,
    pub buy_budget: String,
    pub sell_price_low: String,
    pub sell_price_marginal: String,
    pub sell_price_high: String,
    pub sell_budget: String,
    pub spread_ppm: String,
    pub encoded: String,
}

impl serde::Serialize for Order {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Order", 4)?;
        s.serialize_field("y", &self.y)?;
        s.serialize_field("z", &self.z)?;
        s.serialize_field("A", &self.A)?;
        s.serialize_field("B", &self.B)?;
        s.end()
    }
}

impl serde::Serialize for Strategy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Strategy", 5)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("owner", &self.owner)?;
        s.serialize_field("tokens", &self.tokens)?;
        s.serialize_field("orders", &self.orders)?;
        s.end()
    }
}

pub fn parse_strategy(
    strategy: Strategy,
    strategy_token_decimals: [u8; 2],
) -> Result<ParsedStrategy> {
    let strategy = decode_strategy(strategy)?;
    let DecodedStrategy {
        id,
        token0,
        token1,
        order0,
        order1,
        encoded,
    } = strategy;
    let [decimals0, decimals1] = strategy_token_decimals;
    let buy_price_low = normalize_rate(order1.lowest_rate, decimals0, decimals1)?;
    let buy_price_marginal = normalize_rate(order1.marginal_rate, decimals0, decimals1)?;
    let buy_price_high = normalize_rate(order1.highest_rate, decimals0, decimals1)?;
    let sell_price_low = normalize_inverted_rate(order0.highest_rate, decimals1, decimals0)?;
    let sell_price_marginal = normalize_inverted_rate(order0.marginal_rate, decimals1, decimals0)?;
    let sell_price_high = normalize_inverted_rate(order0.lowest_rate, decimals1, decimals0)?;

    let liquidity0 = order0.liquidity.parse::<U256>()?;
    let liquidity1 = order1.liquidity.parse::<U256>()?;

    let sell_budget = format_units(liquidity0, decimals0)?;
    let buy_budget = format_units(liquidity1, decimals1)?;

    let buy_max = buy_price_high.parse::<f64>()?;
    let sell_max = sell_price_high.parse::<f64>()?;

    let spread_ppm = {
        let spread_ppm = (sell_max / buy_max - 1.0) * 100.0;
        format!("{:.2}", spread_ppm)
    };

    Ok(ParsedStrategy {
        id,
        base_token: token0,
        quote_token: token1,
        buy_price_low,
        buy_price_marginal,
        buy_price_high,
        buy_budget,
        sell_price_low,
        sell_price_marginal,
        sell_price_high,
        sell_budget,
        spread_ppm,
        encoded,
    })
}

fn decode_strategy(strategy: Strategy) -> Result<DecodedStrategy> {
    let str_encoded = serde_json::to_string(&strategy)
        .map_err(|e| eyre!("Failed to serialize strategy: {}", e))?;
    let tokens = strategy.tokens;
    let orders = strategy.orders;

    let order0 = decode_order(orders[0].clone())?;
    let order1 = decode_order(orders[1].clone())?;

    Ok(DecodedStrategy {
        id: strategy.id.to_string(),
        token0: tokens[0],
        token1: tokens[1],
        order0,
        order1,
        encoded: str_encoded,
    })
}

fn decode_order(order: Order) -> Result<DecodedOrder> {
    let y = order.y;
    let z = order.z;
    let a = decode_float(order.A)?;
    let b = decode_float(order.B)?;

    let liquidity = y.to_string();
    let lowest_rate = decode_rate(b);
    let highest_rate = decode_rate(b + a);
    let marginal_rate = if y == z {
        decode_rate(b + a)
    } else {
        let a = U256::from(a);
        let y = U256::from(y);
        let z = U256::from(z);
        let res = a * y / z;
        decode_rate(b + res.to::<u128>())
    };

    Ok(DecodedOrder {
        liquidity,
        lowest_rate,
        highest_rate,
        marginal_rate,
    })
}

const ONE: u64 = 2_u64.pow(48);

pub fn decode_float(value: u64) -> Result<u128> {
    let value = Decimal::from(value);
    let one = Decimal::from(ONE);
    let f = value % one;
    let number_of_bits = value / one;
    let f = f.to_u128().ok_or(eyre!("Failed to convert f to u128"))?;
    let number_of_bits = number_of_bits
        .to_u128()
        .ok_or(eyre!("Failed to convert number_of_bits to u128"))?;
    Ok(f << number_of_bits)
}

fn decode_rate(value: u128) -> String {
    let value = Decimal::from(value);
    (value / Decimal::from(ONE)).powf(2.0).to_string()
}

fn normalize_rate(
    amount: String,
    amount_token_decimals: u8,
    other_token_decimals: u8,
) -> Result<String> {
    let amount = Decimal::from_str(&amount)?;
    let ten_pow = ten_pow(amount_token_decimals, other_token_decimals);
    let amount = amount.mul(ten_pow);
    Ok(amount.to_string())
}

fn normalize_inverted_rate(
    amount: String,
    amount_token_decimals: u8,
    other_token_decimals: u8,
) -> Result<String> {
    let amount = Decimal::from_str(&amount)?;
    if amount.eq(&Decimal::from(0)) {
        return Ok(String::from("0"));
    }
    let amount = Decimal::from(1).div(amount);
    let ten_pow = ten_pow(other_token_decimals, amount_token_decimals);
    let amount = amount.mul(ten_pow);
    Ok(amount.to_string())
}

fn ten_pow(dec0: u8, dec1: u8) -> Decimal {
    let diff = dec0 - dec1;
    Decimal::from(10).powi(diff.into())
}

pub fn is_overlapping_strategy(strategy: &ParsedStrategy) -> Result<bool> {
    let buy_max = Decimal::from_str(&strategy.buy_price_high)?;
    let sell_min = Decimal::from_str(&strategy.sell_price_low)?;

    if sell_min.eq(&Decimal::from(0)) {
        return Ok(false);
    }
    if buy_max.eq(&Decimal::from(0)) {
        return Ok(false);
    }
    Ok(buy_max.ge(&sell_min))
}
