//! { "cases": [ {
//!     "name": "main",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "42"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "50"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.5.0;

contract Test {
    uint8 constant SIZE_HALVED = 2;

    function main(uint8 value) public pure returns(uint8) {
        uint8[SIZE_HALVED * 2] memory array;
        for(uint8 i = 0; i < SIZE_HALVED * 2; i++) {
            array[i] = 2;
        }
        for(uint8 i = 0; i < SIZE_HALVED * 2; i++) {
            value += array[i];
        }
        return value;
    }
}
