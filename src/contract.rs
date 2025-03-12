use alloy::sol;

sol! {
    type Token is address;

    struct Order {
        uint128 y;
        uint128 z;
        uint64 A;
        uint64 B;
    }

    struct Strategy {
        uint256 id;
        address owner;
        Token[2] tokens;
        Order[2] orders;
    }
}
