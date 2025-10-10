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
//!         "60"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    function entry() public pure returns(uint64) {
        return test(32) + test(33);
    }

    function test(uint8 param) private pure returns(uint64) {
        uint64 cnt = 0;

        uint8 p = 0;
        do {
            p += 2;
            if (p % 8 == 0)
                break;
            if (param == 12)
                continue;
            for (uint8 k = 0; k <= 1; k++) {
                for (uint8 x = 0; x <= 1; x++) {
                    uint8 y = 10;
                    while (y < 17) {
                        y += 1;
                        if (y % 3 == 0)
                            continue;
                        if (y == 16)
                            break;
                        if (param == 32 && y > 13)
                            break;
                        cnt += 1;
                    }
                }
            }
        } while (true);

        return cnt;
    }
}
