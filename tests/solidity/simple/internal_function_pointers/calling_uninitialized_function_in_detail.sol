//! { "cases": [ {
//!     "name": "default",
//!     "inputs": [
//!         {
//!             "method": "t",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!     {
//!         "compiler_version": ">=0.8",
//!         "return_data": [
//!             "0x4e487b7100000000000000000000000000000000000000000000000000000000",
//!             "0x0000005100000000000000000000000000000000000000000000000000000000"
//!         ],
//!         "exception": true
//!     },
//!     {
//!         "compiler_version": "<=0.7",
//!         "return_data": [],
//!         "exception": true
//!     }
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.12;

contract Test {
    function() returns (uint256) internal x;
    int256 mutex;

    function t() public returns (uint256) {
        if (mutex > 0) {
            assembly {
                mstore(0, 7)
                return(0, 0x20)
            }
        }
        mutex = 1;
        // Avoid re-executing this function if we jump somewhere.
        x();
        return 2;
    }
}

// ====
// compileToEwasm: also
// ----
// t() -> FAILURE, hex"4e487b71", 0x51
