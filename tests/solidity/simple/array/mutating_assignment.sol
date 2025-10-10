//! { "cases": [ {
//!     "name": "main",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "10", "20", "30", "40", "50", "6"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "218"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.5.0;

contract Test {
    uint8 constant SIZE = 5;

    function main(uint8[SIZE] memory array, uint8 witness) public pure returns(uint8) {
        array[0] += witness;
        array[1] -= witness;
        array[2] *= witness;
        array[3] /= witness;
        array[4] %= witness;

        return array[0] + array[1] + array[2] + array[3] + array[4];
    }
}
