//! { "cases": [ {
//!     "name": "main",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "2"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "42"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    function main(uint8 witness) public pure returns(uint8) {
        uint8 result;
        assembly {
            switch add(witness, mul(witness, witness))
            case 1 { result := 10 }
            case 2 { result := 20 }
            default {
                switch add(witness, mul(witness, witness))
                case 6 { result := 42 }
                default { result := 64 }
            }
        }
        return result;
    }
}
