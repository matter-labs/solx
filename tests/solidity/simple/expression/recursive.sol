//! { "modes": [ "Y", "E" ], "cases": [ {
//!     "name": "default",
//!     "inputs": [
//!         {
//!             "method": "f",
//!             "calldata": [
//!                 "5"
//!             ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [
//!             "0x0100000000"
//!         ]
//!     }
//! } ] }

// SPDX-License-Identifier: MIT

// Report https://linear.app/matterlabs/issue/CPR-737/fix-a-test-failed-after-changing-the-inliner-configuration

pragma solidity >=0.4.12;

contract Test {
    function f(uint x) public returns (uint256 r) {
        return x == 0 ? 2 : f(x - 1)**2;
    }
}
