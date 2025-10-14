//! { "cases": [ {
//!     "name": "zero",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "0"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! }, {
//!     "name": "ordinar",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "42"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "42"
//!     ]
//! }, {
//!     "name": "max",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "255"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "255"
//!     ]
//! }, {
//!     "name": "overflow_negative_minimal",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "-1"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "255"
//!     ]
//! }, {
//!     "name": "overflow_negative",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "-42"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "214"
//!     ]
//! }, {
//!     "name": "overflow_positive_minimal",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "256"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! }, {
//!     "name": "overflow_positive",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "1000"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "232"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    function main(int256 a) public pure returns(uint8) {
        return uint8(uint256(a));
    }
}
