//! { "cases": [ {
//!     "name": "default",
//!     "inputs": [
//!         {
//!             "method": "#deployer",
//!             "caller": "0x0000000000000000000000000000000000000099",
//!             "calldata": [
//!                 "25", "42"
//!             ],
//!             "expected": [
//!                 "Test.address"
//!             ]
//!         },
//!         {
//!             "method": "a",
//!             "calldata": [],
//!             "caller": "0x0000000000000000000000000000000000000099",
//!             "expected": [ "25" ]
//!         },
//!         {
//!             "method": "b",
//!             "calldata": [],
//!             "caller": "0x0000000000000000000000000000000000000099",
//!             "expected": [ "0x99" ]
//!         },
//!         {
//!             "method": "c",
//!             "calldata": [],
//!             "caller": "0x0000000000000000000000000000000000000099",
//!             "expected": [ "42" ]
//!         }
//!     ],
//!     "expected": [ "42" ]
//! } ] }

// SPDX-License-Identifier: MIT

// Report https://linear.app/matterlabs/issue/CPR-288/implement-the-setimmutable-and-loadimmutable-functions

pragma solidity >=0.8.0;

contract Test {
    uint immutable public a;
    address immutable public b;
    uint8 immutable public c;

    constructor(uint x, uint8 y) {
        a = x;
        b = msg.sender;
        c = y;
    }
}
