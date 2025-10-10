//! { "modes": [ "Y", "E" ], "cases": [ {
//!     "name": "zero",
//!     "inputs": [
//!         {
//!             "method": "fact",
//!             "calldata": [
//!                 "0"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! }, {
//!     "name": "fifth",
//!     "inputs": [
//!         {
//!             "method": "fact",
//!             "calldata": [
//!                 "5"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "120"
//!     ]
//! }, {
//!     "name": "twelfth",
//!     "inputs": [
//!         {
//!             "method": "fact",
//!             "calldata": [
//!                 "12"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "479001600"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    function fact(uint8 n) public pure returns(uint64) {
        if (n == 0) {
            return 1;
        }

        return fact(n - 1) * n;
    }
}
