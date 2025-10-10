//! { "cases": [ {
//!     "name": "main",
//!     "inputs": [
//!         {
//!             "method": "#deployer",
//!             "calldata": [
//!                 "0x03",
//!                 "0x05",
//!                 "0x02"
//!             ],
//!             "expected": [
//!                 "Test.address"
//!             ]
//!         },
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "12"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "42"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.8.0;

contract Test {
    uint256 immutable field_1;
    uint256 immutable field_2;
    uint256 immutable field_3;

    constructor(uint256 a, uint256 b, uint256 c) {
        field_1 = a;
        field_2 = b;
        field_3 = c;
    }

    function main(uint8 witness) external view returns(uint8) {
        return witness + uint8(field_1) * uint8(field_2) * uint8(field_3);
    }
}
