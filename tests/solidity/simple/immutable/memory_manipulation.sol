//! { "modes": [
//!     "Y", "E+ <=0.8.12", "E+ >=0.8.15", "E-", "I"
//! ], "cases": [
//!         {
//!             "name": "main",
//!             "inputs": [
//!                 {
//!                     "method": "main",
//!                     "calldata": []
//!                 }
//!             ],
//!             "expected": {
//!                 "return_data": ["2"]
//!             }
//!         }
//!     ]
//! }

// SPDX-License-Identifier: MIT

pragma solidity >=0.8.0;

contract Test {
    uint256 public immutable X;

    constructor() public {
        X = 1;
        assembly {
            mstore(0x80, 2)
        }
    }

    function main() public view returns (uint256) {
        return X;
    }
}
