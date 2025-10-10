//! { "cases": [ {
//!     "name": "entry",
//!     "inputs": [
//!         {
//!             "method": "entry",
//!             "calldata": [
//!                 "31"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1008"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    function entry(uint8 param) public pure returns(uint64) {
        uint64 cnt = 0;

        uint8 j = 1;
        while (j < 4) {
            uint8 p = 0;
            do {
                p += 2;
                if (p%8 == 0)
                    break;
                for (uint8 h = 1; h <= 4; h++) {
                    if (param == 4)
                        return 40;
                    if (h > 2)
                        break;
                    for (uint8 k = 10; k < 12; k++) {
                        uint8 x = 7;
                        do {
                            x -= 1;
                            if (x == 0)
                                break;
                            uint8 y = 10;
                            while (y < 17) {
                                y += 1;
                                cnt += 1;
                            }
                        } while (true);
                    }
                }
            } while (true);
            j *= 2;
        }
        
        return cnt;
    }
}
