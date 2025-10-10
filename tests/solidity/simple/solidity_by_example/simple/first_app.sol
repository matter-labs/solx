//! { "cases": [ {
//!     "name": "init_value",
//!     "inputs": [
//!         {
//!             "method": "count",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! }, {
//!     "name": "first",
//!     "inputs": [
//!         {
//!             "method": "inc",
//!             "calldata": [
//!             ]
//!         }, {
//!             "method": "count",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! }, {
//!     "name": "second",
//!     "inputs": [
//!         {
//!             "method": "inc",
//!             "calldata": [
//!             ]
//!         }, {
//!             "method": "inc",
//!             "calldata": [
//!             ]
//!         }, {
//!             "method": "dec",
//!             "calldata": [
//!             ]
//!         }, {
//!             "method": "count",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT
pragma solidity >=0.4.16;

contract Test {
    uint public count;

    // Function to get the current count
    function get() public view returns (uint) {
        return count;
    }

    // Function to increment count by 1
    function inc() public {
        count += 1;
    }

    // Function to decrement count by 1
    function dec() public {
        count -= 1;
    }
}
