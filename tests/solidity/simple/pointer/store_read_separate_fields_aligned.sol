//! { "cases": [ {
//!     "name": "one",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "1", "2", "3"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "6"
//!     ]
//! }, {
//!     "name": "two",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "42", "21", "84"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "147"
//!     ]
//! }, {
//!     "name": "three",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "99", "66", "33"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "198"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    struct Data {
        uint256 a;
        uint256 b;
        uint256 c;
    }

    function main(uint256 a, uint256 b, uint256 c) public pure returns(uint256 result) {
        Data memory data = Data(a, b, c);
        assembly {
            result := add(mload(data), add(mload(add(data, 0x20)), mload(add(data, 0x40))))
        }
    }
}