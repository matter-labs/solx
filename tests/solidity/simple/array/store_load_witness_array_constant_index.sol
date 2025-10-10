//! { "cases": [ {
//!     "name": "main",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "1", "2", "3", "4"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "64"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.5.0;

contract Test {
    uint8 constant SIZE = 4;

    function main(uint8[SIZE] memory array) public pure returns(uint8) {
        array[3] = 106;
        array[1] = 42;

        return array[3] - array[1];
    }
}
