//! { "cases": [ {
//!     "name": "entry",
//!     "inputs": [
//!         {
//!             "method": "entry",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "5"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    function entry() public pure returns(uint64) {
        uint64 cnt = 0;
        for(uint8 i = 0; i < 12; i++) {
            if (i % 2 == 0) {
                continue;
            }
            cnt += 1;
            if (i == 9) {
                break;
            }
        }
        return cnt;
    }
}
