//! { "cases": [ {
//!     "name": "main",
//!     "inputs": [
//!         {
//!             "method": "f",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0",
//!         "2",
//!         "3"
//!     ]
//! } ] }
// SPDX-License-Identifier: MIT

pragma solidity >=0.4.12;

contract Test {
    uint8 public a;

    function f() public returns (uint8[3] memory) {
        uint8[3] memory arr = [a, f1(), 3];
        return arr;
    }

    function f1() public returns (uint8) {
        a = 5;
        return 2;
    }
}
