//! { "cases": [ {
//!     "name": "default",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": []
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.8.14;

contract Test {
    function main() external view returns (uint256) {
        uint256 a = 2; // any value >= 2 seems to cause an overflow
        return this.inner(a, true);
    }

    function inner(uint256 a, bool b) external pure returns (uint256) {
        uint256 c = b ? 1 : 0;
        uint256 d = a + c; // this arithmetic incorrectly throws an overflow panic
        return b ? c : d;
    }
}
