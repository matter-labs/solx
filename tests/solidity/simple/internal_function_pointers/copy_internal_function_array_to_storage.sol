//! { "cases": [ {
//!     "name": "default",
//!     "inputs": [
//!         {
//!             "method": "one",
//!             "calldata": [],
//!             "expected": [ "3" ]
//!         },
//!         {
//!             "method": "two",
//!             "calldata": []
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
    function() internal returns (uint)[20] x;
    int256 mutex;

    function one() public returns (uint256) {
        function() internal returns (uint)[20] memory xmem;
        x = xmem;
        return 3;
    }

    function two() public returns (uint256) {
        if (mutex > 0) return 7;
        mutex = 1;
        // If this test fails, it might re-execute this function.
        x[0]();
        return 2;
    }
}

// ====
// compileToEwasm: also
// ----
// one() -> 3
// gas legacy: 140253
// gas legacyOptimized: 140093
// two() -> FAILURE, hex"4e487b71", 0x51
