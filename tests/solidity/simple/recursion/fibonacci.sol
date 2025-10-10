//! { "modes": [ "Y", "E" ], "cases": [ {
//!     "name": "first",
//!     "inputs": [
//!         {
//!             "method": "fib",
//!             "calldata": [
//!                 "1"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! }, {
//!     "name": "eighth",
//!     "inputs": [
//!         {
//!             "method": "fib",
//!             "calldata": [
//!                 "8"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "21"
//!     ]
//! }, {
//!     "name": "sixteenth",
//!     "inputs": [
//!         {
//!             "method": "fib",
//!             "calldata": [
//!                 "16"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "987"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    function fib(uint8 n) public pure returns(uint64) {
        if (n <= 1) {
            return n;
        }

        return fib(n - 1) + fib(n - 2);
    }
}
