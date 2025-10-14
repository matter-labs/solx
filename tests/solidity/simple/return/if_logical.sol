//! { "cases": [ {
//!     "name": "entry1",
//!     "inputs": [
//!         {
//!             "method": "entry",
//!             "calldata": [
//!                 "0", "0"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "4"
//!     ]
//! }, {
//!     "name": "false_true",
//!     "inputs": [
//!         {
//!             "method": "entry",
//!             "calldata": [
//!                 "0", "1"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "8"
//!     ]
//! }, {
//!     "name": "true_false",
//!     "inputs": [
//!         {
//!             "method": "entry",
//!             "calldata": [
//!                 "1", "0"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "8"
//!     ]
//! }, {
//!     "name": "true_true",
//!     "inputs": [
//!         {
//!             "method": "entry",
//!             "calldata": [
//!                 "1", "1"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "4"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    function entry(bool first, bool second) public pure returns(uint8) {
        uint8 result = 1;

        if (first || second) {
            result <<= 1;
        }

        if (first != second) {
            result <<= 1;
        }

        if (first && second) {
            result <<= 1;
        }

        if (!first) {
            result <<= 1;
        }

        if (!second) {
            result <<= 1;
        }

        return result;
    }
}
