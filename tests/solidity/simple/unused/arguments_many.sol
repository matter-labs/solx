//! { "cases": [ {
//!     "name": "main",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "1", "2", "3", "4", "5", "6"
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
    function main(uint256 a, uint256 b, uint256 c, uint256 d, uint256 e, uint256 f) public pure returns(uint256) {
        return d;
    }
}
