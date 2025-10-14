//! { "cases": [ {
//!     "name": "main",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "$CHAIN_ID"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.5.14;

contract Test {
    function main() public returns(uint) {
        uint chainId;
        assembly {
            chainId := chainid()
        }
        return chainId;
    }
}
