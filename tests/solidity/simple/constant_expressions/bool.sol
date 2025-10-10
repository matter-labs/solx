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
//! }, {
//!     "name": "test2",
//!     "inputs": [
//!         {
//!             "method": "test2",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    function test() public pure returns (bool) {
        return !false || true && false;
    }

    function test2() public pure returns (bool) {
        return !false == true != true;
    }
}
