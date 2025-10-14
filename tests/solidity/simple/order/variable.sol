//! { "cases": [ {
//!     "name": "remainder_zero",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "15"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "5"
//!     ]
//! }, {
//!     "name": "remainder_one",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "22"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "7"
//!     ]
//! }, {
//!     "name": "remainder_two",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "26"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "8"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    function main(uint32 witness) public pure returns(uint32) {
        uint32 variable = 3;
        return witness / variable;
    }
}
