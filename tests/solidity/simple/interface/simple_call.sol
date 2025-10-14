//! { "cases": [ {
//!     "name": "default",
//!     "inputs": [
//!         {
//!             "method": "entry",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

// Report https://linear.app/matterlabs/issue/CPR-581/require-in-external-call-failure-for-b1-b3

pragma solidity >=0.6.0;

contract Test {
    function f() public pure {
        require(true);
    }

    function entry() public view {
        this.f();
    }
}
