# Carbon DeFi Decode

A simple library to decode Carbon Protocol's strategies.

## Overview

Carbon Decode is a Rust library designed to parse and interpret Carbon Protocol's strategy data. It provides tools to decode and analyze Carbon's decentralized finance strategies.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
carbon-decode = "0.1.0"
```

## Features

- Decode Carbon Protocol strategy data
- Parse strategy parameters
- Convert raw data into usable Rust structs

## Usage

```rust
use carbon-decode::decode::parse_strategy;

let parsed_strategy = parse_strategy(strategy);
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Links

- [GitHub Repository](https://github.com/t0rbik/carbon-decode)
- [Carbon SDK](https://github.com/bancorprotocol/carbon-sdk)
