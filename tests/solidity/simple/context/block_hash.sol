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
//!         "$BLOCK_HASH:1"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.22;

contract Test {
    function main() public returns(bytes32) {
        bytes32 _blockhash = blockhash(block.number - 1);
        return _blockhash;
    }
}
