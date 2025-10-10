//! { "cases": [ {
//!     "name": "main",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "16"
//!             ],
//!             "storage": { "Test.address": [
//!                 "3", "5", "2"
//!             ] }
//!         }
//!     ],
//!     "expected": [
//!         "81"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    uint256 field_1;
    uint256 field_2;
    uint256 field_3;

    function main(uint8 witness) public returns(uint8) {
        return 19 * 3 - 8 / uint8(field_1) + (witness / (uint8(field_2) - 3) + 5) * (8 / uint8(field_3) / 2);
    }
}
