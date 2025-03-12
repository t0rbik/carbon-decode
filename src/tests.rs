use alloy::primitives::{Address, U256};

use crate::contract::{Order, Strategy};

use crate::decode::{decode_float, is_overlapping_strategy, parse_strategy};

#[test]
fn decode_float_test() {
    let float = decode_float(("3562339608471328").parse::<u64>().unwrap()).unwrap();
    assert_eq!(float.to_string(), String::from("756284981016395776"))
}

#[test]
fn test_parse_strategy_and_get_spread_ppm_mock1() {
    let strategy = Strategy {
        id: "340282366920938463463374607431768211534"
            .parse::<U256>()
            .unwrap(),
        owner: "0xC5597Eb414B65F4e905Af8f45Ff95E2e22D1E4b0"
            .parse::<Address>()
            .unwrap(),
        tokens: [
            "0x4200000000000000000000000000000000000006"
                .parse::<Address>()
                .unwrap(),
            "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913"
                .parse::<Address>()
                .unwrap(),
        ],
        orders: [
            Order {
                y: "443317550496385".parse::<u128>().unwrap(),
                z: "870293078344329".parse::<u128>().unwrap(),
                A: "3544951559321984".parse::<u64>().unwrap(),
                B: "4381635731557859".parse::<u64>().unwrap(),
            },
            Order {
                y: "1020804".parse::<u128>().unwrap(),
                z: "2218082".parse::<u128>().unwrap(),
                A: "1751224833".parse::<u64>().unwrap(),
                B: "13361396492".parse::<u64>().unwrap(),
            },
        ],
    };

    let strategy = parse_strategy(strategy, [18, 6]).unwrap();

    assert_eq!(strategy.spread_ppm, "0.60");
    assert_eq!(is_overlapping_strategy(&strategy).unwrap(), true);
}

#[test]
fn test_parse_strategy_and_get_spread_ppm_mock2() {
    let strategy = Strategy {
        id: "6465364971497830805804117541203596017740"
            .parse::<U256>()
            .unwrap(),
        owner: "0x2998166a1c40f91617a343071af67183df37f43d"
            .parse::<Address>()
            .unwrap(),
        tokens: [
            "0xd386a121991e51eab5e3433bf5b1cf4c8884b47a"
                .parse::<Address>()
                .unwrap(),
            "0x833589fcd6edb6e08f4c7c32d4f71b54bda02913"
                .parse::<Address>()
                .unwrap(),
        ],
        orders: [
            Order {
                y: "2419891880285895608".parse::<u128>().unwrap(),
                z: "12965995751316401403".parse::<u128>().unwrap(),
                A: "6367105957745619".parse::<u64>().unwrap(),
                B: "6404666349060909".parse::<u64>().unwrap(),
            },
            Order {
                y: "500000".parse::<u128>().unwrap(),
                z: "709135".parse::<u128>().unwrap(),
                A: "40123691".parse::<u64>().unwrap(),
                B: "48752896".parse::<u64>().unwrap(),
            },
        ],
    };

    let strategy = parse_strategy(strategy, [18, 6]).unwrap();

    assert_eq!(strategy.spread_ppm, "0.30");
    assert_eq!(is_overlapping_strategy(&strategy).unwrap(), true);
}

#[test]
fn test_parse_strategy_and_get_spread_ppm_mock3() {
    let strategy = Strategy {
        id: "1020847100762815390390123822295304634374"
            .parse::<U256>()
            .unwrap(),
        owner: "0x069e85D4F1010DD961897dC8C095FBB5FF297434"
            .parse::<Address>()
            .unwrap(),
        tokens: [
            "0xd386a121991e51eab5e3433bf5b1cf4c8884b47a"
                .parse::<Address>()
                .unwrap(),
            "0x4200000000000000000000000000000000000006"
                .parse::<Address>()
                .unwrap(),
        ],
        orders: [
            Order {
                y: "5272165526976235778".parse::<u128>().unwrap(),
                z: "5272165526976235778".parse::<u128>().unwrap(),
                A: "137699263369260".parse::<u64>().unwrap(),
                B: "785475461108442".parse::<u64>().unwrap(),
            },
            Order {
                y: "0".parse::<u128>().unwrap(),
                z: "20000000000000000".parse::<u128>().unwrap(),
                A: "15495486182908".parse::<u64>().unwrap(),
                B: "68947006830288".parse::<u64>().unwrap(),
            },
        ],
    };

    let strategy = parse_strategy(strategy, [18, 18]).unwrap();

    assert_eq!(strategy.spread_ppm, "11.11");
    assert_eq!(is_overlapping_strategy(&strategy).unwrap(), true);
}
