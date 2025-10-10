//! { "cases": [ {
//!     "name": "main",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "16"
//!             ],
//!             "storage": { "Test.address": [
//!                 "0x020503"
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
    uint8 field_1;
    uint8 field_2;
    uint8 field_3;

    function main(uint8 witness) public returns(uint8) {
        return 19 * 3 - 8 / field_1 + (witness / (field_2 - 3) + 5) * (8 / field_3 / 2);
    }
}
