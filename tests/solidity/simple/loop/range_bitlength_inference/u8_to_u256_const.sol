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
//!         "55"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    uint256 constant LIMIT = 10;

    function main() public pure returns(uint256) {
        uint256 sum = 0;
        for(uint256 i = 0; i <= LIMIT; i++) {
            sum += i;
        }
        return sum;
    }
}
