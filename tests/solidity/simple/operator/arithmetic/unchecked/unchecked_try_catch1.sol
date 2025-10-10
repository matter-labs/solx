//! { "cases": [ {
//!     "name": "main",
//!     "inputs": [
//!         {
//!             "method": "f1",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.8.0;

contract Test {
    function f1() public returns (uint256) {
        unchecked {
            try this.f2() {
                return f2() + 1;
            } catch {}
        }
    }

    function f2() public returns (uint256) {
        uint256 b = 2 ** 256 - 1;
        return b;
    }
}
