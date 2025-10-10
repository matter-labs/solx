//! { "cases": [ {
//!     "name": "default",
//!     "inputs": [
//!         {
//!             "caller": "$TX_ORIGIN",
//!             "method": "main",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "$TX_ORIGIN"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    function main() external view returns(uint256 result) {
        assembly {
            result := origin()
        }
    }
}
