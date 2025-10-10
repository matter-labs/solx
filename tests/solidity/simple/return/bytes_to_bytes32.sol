//! { "cases": [ {
//!     "name": "test",
//!     "inputs": [
//!         {
//!             "method": "verify",
//!             "calldata": [
//!                 "0x20",
//!                 "32",
//!                 "0"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

// Report https://linear.app/matterlabs/issue/CPR-450/vyper-extract32-exception-without-optimization

pragma solidity >=0.8.5;

contract Test {
    function verify(bytes calldata sig) external returns(bytes32) {
        return bytes32(sig);
    }
}
