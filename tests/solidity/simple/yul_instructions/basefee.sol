//! { "cases": [ {
//!     "name": "default",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "$BASE_FEE"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.8.7;

contract Test {
    function main() external view returns(uint256 result) {
        assembly {
            result := basefee()
        }
    }
}
