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

pragma solidity >=0.4.16 <=0.8.17;

contract Test {
    function main() external view returns(uint256 result) {
        assembly {
            result := difficulty()
        }
    }
}
