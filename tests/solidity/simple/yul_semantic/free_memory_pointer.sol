//! { "modes": [
//!     "Y",
//!     "E- >=0.5.5",
//!     "E+ >=0.8.15",
//!     "I"
//! ], "cases": [ {
//!     "name": "main",
//!     "inputs": [
//!         {
//!             "method": "test",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [],
//!         "exception": true
//!     }
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.12;

contract Test {
    function test() external pure returns (uint ret) {
        assembly {
            mstore(64, 1000000000000)
        }
        ret = 42;
    }
}
