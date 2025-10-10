//! { "cases": [ {
//!     "name": "one",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "84", "21"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "21"
//!     ]
//! }, {
//!     "name": "two",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "42", "42"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "42"
//!     ]
//! }, {
//!     "name": "three",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "21", "84"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "84"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    function main(uint16 _address, uint8 value) public pure returns(uint8 result) {
        uint16 pointer = _address * 32;
        assembly {
            mstore(pointer, value)
            result := mload(pointer)
        }
    }
}