//! { "cases": [ {
//!     "name": "_default",
//!     "inputs": [
//!         {
//!             "method": "_default",
//!             "calldata": [
//!                 "512"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1024"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    function _default(uint256 value) public pure returns(uint256) {
        return value * 2;
    }
}