//! { "cases": [ {
//!     "name": "one",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "1", "0", "0"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! }, {
//!     "name": "two",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "0", "1", "1"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "2"
//!     ]
//! }, {
//!     "name": "three",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "0", "1", "0"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "3"
//!     ]
//! }, {
//!     "name": "four",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "0", "0", "0"
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
    function main(bool[3] memory gates) public pure returns(uint8) {
       if (gates[0]) {
           return 1;
       } else if (gates[1]) {
           if (gates[2]) {
               return 2;
           } else {
               return 3;
           }
       } else {
           return 4;
       }
    }
}
