//! { "cases": [ {
//!     "name": "entry0",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "0"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "4"
//!     ]
//! }, {
//!     "name": "entry1",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "1"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "5"
//!     ]
//! }, {
//!     "name": "entry2",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "2"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "6"
//!     ]
//! }, {
//!     "name": "entry3",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "3"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "7"
//!     ]
//! }, {
//!     "name": "entry4",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "7"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "99"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    function main(uint8 param) public pure returns(uint64) {
        if (param < 3) {
            if (param < 2) {
                if (param == 0) {
                    return 4;
                } else {
                    return 5 * param; // 5
                }
            }
            return param * 3; // 6
        } else {
            if (param == 3) {
                return param * 2 + 1; // 7
            }
            return 99;
        }
    }
}