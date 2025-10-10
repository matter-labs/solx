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
//!         "$PREVRANDAO"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.8.18;

contract Test {
    function main() external view returns(uint256 result) {
        assembly {
            result := prevrandao()
        }
    }
}
