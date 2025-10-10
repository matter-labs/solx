//! { "cases": [ {
//!     "name": "main",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "31"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "2"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

// Report https://linear.app/matterlabs/issue/CPR-206/back-end-optimization-bug-minimized-complex-wrong-result

pragma solidity >=0.4.16;

contract Test {
    function main(uint8 param) public pure returns(uint64) {
        uint64 cnt = 0;

        uint8 j = 0;
        while (j < 2) {
            for (uint8 h = 0; h < 1; h++) {
                if (param == 4)
                    return 40;
                cnt += 1;
            }
            j += 1;
        }
        
        return cnt;
    }
}
