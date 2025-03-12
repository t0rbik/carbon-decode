# Carbon DeFi Decode

A simple library to decode Carbon Protocol's strategies.

## Overview

Carbon DeFi Decode is a Rust library designed to parse and interpret Carbon Protocol's strategy data.
It uses alloy under the hood.

## Installation

Run this command in your project

```bash
cargo add carbon-defi-decode
```

## Features

- Decode Carbon Protocol strategy data
- Parse strategy parameters
- Convert raw data into usable Rust structs

## Usage

```rust
use carbon-defi-decode::decode::parse_strategy;

let parsed_strategy = parse_strategy(strategy);
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Links

- [GitHub Repository](https://github.com/t0rbik/carbon-decode)
- [Carbon SDK](https://github.com/bancorprotocol/carbon-sdk)
