//! { "cases": [ {
//!     "name": "main",
//!     "inputs": [
//!         {
//!             "method": "#deployer",
//!             "calldata": [
//!                 "32"
//!             ],
//!             "expected": [
//!                 "Test.address"
//!             ]
//!         },
//!         {
//!             "method": "_get",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

// Report https://linear.app/matterlabs/issue/CPR-296/bug-with-constructor

pragma solidity >=0.7.0;

contract Test {
    uint public a;

    // if remove constructor or param - it works
    constructor(uint b) {
        a = 0;
    }

    function _get() public view returns(uint) {
        return a;
    }
}