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
    function _default(uint248 value) public pure returns(uint248) {
        return value * 2;
    }
}
