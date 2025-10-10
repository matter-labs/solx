//! { "cases": [ {
//!     "name": "main",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "16"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "128"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    struct Data {
        uint8[3] a;
    }

    function main(uint8 witness) public pure returns(uint8) {
        Data[3] memory data = [
            Data([1, 2, 3]),
            Data([3, 4, 5]),
            Data([7, 8, 9])
        ];

        return data[2].a[1] * witness;
    }
}
