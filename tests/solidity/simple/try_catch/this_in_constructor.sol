//! {
//!     "cases": [
//!         {
//!             "name": "main",
//!             "inputs": [
//!                 {
//!                     "method": "#deployer",
//!                     "calldata": []
//!                 }
//!             ],
//!             "expected": {
//!                 "return_data": [],
//!                 "exception": true
//!             }
//!         }
//!     ]
//! }

// SPDX-License-Identifier: MIT

pragma solidity >=0.6.0;

contract Test {
    constructor() public {
        try this.f1() {} catch {}
    }

    function f1() public {}
}
