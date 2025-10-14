//! { "cases": [ {
//!     "name": "test",
//!     "inputs": [
//!         {
//!             "method": "test",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

// Report https://linear.app/matterlabs/issue/CPR-204/2-dimensional-array-bug

pragma solidity >=0.4.16;

contract Test {
    function test() public pure returns(uint64) {
        uint64[1][1] memory a;
        a[0][0];
        return 1;
    }
}