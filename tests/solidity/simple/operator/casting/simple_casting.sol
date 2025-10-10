//! { "cases": [ {
//!     "name": "first",
//!     "inputs": [
//!         {
//!             "method": "first",
//!             "calldata": [
//!                 "11"
//!             ]
//!         }
//!     ],
//!     "expected": [ {
//!         "compiler_version": ">=0.8",
//!         "return_data": [
//!             "0x4e487b7100000000000000000000000000000000000000000000000000000000",
//!             "0x0000001100000000000000000000000000000000000000000000000000000000"
//!         ],
//!         "exception": true
//!     }, {
//!         "compiler_version": "<0.8",
//!         "return_data": [
//!             "8"
//!         ],
//!         "exception": false
//!     } ]
//! }, {
//!     "name": "second",
//!     "inputs": [
//!         {
//!             "method": "second",
//!             "calldata": [
//!                 "11"
//!             ]
//!         }
//!     ],
//!     "expected": [ {
//!         "return_data": [
//!             "264"
//!         ]
//!     } ]
//! }, {
//!     "name": "third",
//!     "inputs": [
//!         {
//!             "method": "third",
//!             "calldata": [
//!                 "195"
//!             ]
//!         }
//!     ],
//!     "expected": [ {
//!         "compiler_version": ">=0.8",
//!         "return_data": [
//!             "0x4e487b7100000000000000000000000000000000000000000000000000000000",
//!             "0x0000001100000000000000000000000000000000000000000000000000000000"
//!         ],
//!         "exception": true
//!     }, {
//!         "compiler_version": "<0.8",
//!         "return_data": [
//!             "70784"
//!         ],
//!         "exception": false
//!     } ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.12;

contract Test {
    function first(uint8 _num) public returns (uint256) {
        return _num * 24;
    }

    function second(uint8 _num) public returns (uint256) {
        return _num * uint256(24);
    }

    function third(uint8 _num) public returns (uint256) {
        return _num * 1 days;
    }
}
