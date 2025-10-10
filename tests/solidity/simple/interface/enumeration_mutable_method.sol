//! { "cases": [ {
//!     "name": "main",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "8"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "48"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    enum List {
        FIRST,
        SECOND,
        THIRD
    }

    function multiply(List list, List other) private pure returns(uint8) {
        return (uint8(list) + 1) * (uint8(other) + 1);
    }

    function main(uint8 witness) public pure returns(uint8) {
        return multiply(List.SECOND, List.THIRD) * witness;
    }
}
