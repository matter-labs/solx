//! { "cases": [ {
//!     "name": "main",
//!     "inputs": [
//!         {
//!             "method": "#deployer",
//!             "calldata": [
//!                 "0x05",
//!                 "0x07"
//!             ],
//!             "expected": [
//!                 "Test.address"
//!             ]
//!         },
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "42"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "148"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.8.0;

contract Test {
    uint256 constant SOMETHING = 42;
    uint256 constant SOMETHING_ELSE = 88;

    struct Data {
        uint256 c;
        uint256 d;
    }

    uint8 immutable a;
    uint8 immutable b;

    constructor(uint8 x, uint8 y) public {
        a = x;
        b = y;
    }

    function inner(Data memory data, uint256 _value, uint8 literal) internal returns(uint256) {
        return ((a + data.c + b + data.d + _value) * literal * SOMETHING - SOMETHING_ELSE) / 1000;
    }

    function main(uint256 _value) external returns(uint256) {
        Data memory data = Data({c: 10, d: 20});

        return inner(data, _value, 42);
    }
}
