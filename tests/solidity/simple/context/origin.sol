//! { "cases": [ {
//!     "name": "main",
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

pragma solidity >=0.4.12;

contract Test {
    function main() public returns(address) {
        address tx_origin = tx.origin;
        return tx_origin;
    }
}
