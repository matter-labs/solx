//! { "cases": [ {
//!     "name": "complex",
//!     "inputs": [
//!         {
//!             "method": "#deployer",
//!             "calldata": [
//!             ],
//!             "expected": [
//!                 "Test.address"
//!             ]
//!         },
//!         {
//!             "method": "get",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! } ] }

// Report https://linear.app/matterlabs/issue/CPR-285/bug-with-dynamic-arrays

// SPDX-License-Identifier: MIT

pragma solidity >=0.7.0;

contract Test {
    uint[] public a; // with static array works

    constructor() { // if replace this with function(not constructor) - it works
        a.push(1);
    }

    function get() public view returns(uint) {
        return a[0];
    }
}