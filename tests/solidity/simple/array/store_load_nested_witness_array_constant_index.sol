//! { "cases": [ {
//!     "name": "main",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "1", "2", "3", "4",
//!                 "5", "6", "7", "8",
//!                 "9", "10", "11", "12",
//!                 "13", "14", "15", "16"
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
    uint8 constant SIDE_SIZE = 4;

    function main(uint8[SIDE_SIZE][SIDE_SIZE] memory array) public pure returns(uint8) {
        array[3][1] = 106;
        array[1][3] = 42;

        return array[3][1] - array[1][3];
    }
}
