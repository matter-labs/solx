//! { "cases": [ {
//!     "name": "f",
//!     "inputs": [
//!         {
//!             "method": "f",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0x00000000000000000000000000000000000000ff"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.8.8;

contract Test {
    address public immutable a2;
    address public a1 = address(0xaa);

    constructor() {
        a2 = address(0xff);
        a1 = a2;
    }

    function f() public returns (address) {
        return a1;
    }
}
