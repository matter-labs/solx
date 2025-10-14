//! { "cases": [ {
//!     "name": "main",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "25"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "131"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    uint248 constant CONST = 42;

    function main(uint248 witness) public pure returns(uint248) {
        uint248 local = 64;

        return CONST + local + witness;
    }
}
