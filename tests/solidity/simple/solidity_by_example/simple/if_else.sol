//! { "cases": [ {
//!     "name": "foo_1",
//!     "inputs": [
//!         {
//!             "method": "foo",
//!             "calldata": [
//!                 "1"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! }, {
//!     "name": "foo_15",
//!     "inputs": [
//!         {
//!             "method": "foo",
//!             "calldata": [
//!                 "15"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! }, {
//!     "name": "foo_123132213123",
//!     "inputs": [
//!         {
//!             "method": "foo",
//!             "calldata": [
//!                 "123132213123"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "2"
//!     ]
//! }, {
//!     "name": "ternary_9",
//!     "inputs": [
//!         {
//!             "method": "ternary",
//!             "calldata": [
//!                 "1"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! }, {
//!     "name": "ternary_57896044618658097711785492504343953926634992332820282019728792003956564819967",
//!     "inputs": [
//!         {
//!             "method": "ternary",
//!             "calldata": [
//!                 "57896044618658097711785492504343953926634992332820282019728792003956564819967"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "2"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT
pragma solidity >=0.4.16;

contract Test {
    function foo(uint x) public pure returns (uint) {
        if (x < 10) {
            return 0;
        } else if (x < 20) {
            return 1;
        } else {
            return 2;
        }
    }

    function ternary(uint _x) public pure returns (uint) {
        // if (_x < 10) {
        //     return 1;
        // }
        // return 2;

        // shorthand way to write if / else statement
        return _x < 10 ? 1 : 2;
    }
}
