//! { "cases": [ {
//!     "name": "test",
//!     "inputs": [
//!         {
//!             "method": "hashes",
//!             "calldata": [
//!                 "2"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0x10ca3eff73ebec87d2394fc58560afeab86dac7a21f5e402ea0a55e5c8a6758f"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

// Report https://linear.app/matterlabs/issue/CPR-585/merkle-tree-minimization-keccak256-problem-for-m3b0

pragma solidity >=0.7.0;

contract Test {
    bytes32[] public hashes; // If replace with static array - works
    // bytes32 res;

    // If replace constructor with function call - works.
    constructor() {
        // res = keccak256(bytes("")); If uncomment this line(with res declaration) and
        // push at the next line number - it will still not work, so maybe problem with keccak256 calls, not with storage
        hashes.push(keccak256(bytes(""))); // If push here number instead keccak256 - works.
        // res = keccak256(abi.encodePacked("")); The same as above
        hashes.push(keccak256(abi.encodePacked(""))); // If push here number instead keccak256 - works.

        hashes.push(
            keccak256(
                abi.encodePacked(hashes[1])
            )
        ); // If push here number instead keccak256 - works.
    }
}
