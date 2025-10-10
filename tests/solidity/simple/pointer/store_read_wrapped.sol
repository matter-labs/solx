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
    struct Wrapper {
        uint16 pointer;
    }

    function main(uint16 _address, uint8 value) public pure returns(uint8 result) {
        Wrapper memory wrapper = Wrapper(_address * 32);
        assembly {
            mstore(mload(wrapper), value)
            result := mload(mload(wrapper))
        }
    }
}