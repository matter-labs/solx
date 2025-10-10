//! { "cases": [ {
//!     "name": "default",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": []
//!         }
//!     ],
//!     "expected": [
//!         "10"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

// Report https://linear.app/matterlabs/issue/CPR-952/statements-inside-loops-stack-overflow-error-in-the-zksolc

pragma solidity >=0.6.0;

contract Test {
    function main() public view returns (uint result) {
        for (uint i = 0; i < 10; i++) {
            try this.externalCall() returns (uint externalResult) {
                result += externalResult;
            } catch {}
        }
    }

    function externalCall() public pure returns (uint result) {
        result = 1;
    }
}
