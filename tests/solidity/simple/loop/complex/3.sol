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
//!         "3"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    function entry() public pure returns(uint64) {
        return test() + test() + test();
    }

    function test() private pure returns(uint64) {
        for (uint8 i = 0; i < 2; i++) {
            uint8 j = 1;
            while (j < 4) {
                uint8 p = 0;
                do {
                    p += 2;
                    if (p == 8)
                        break;
                    for (uint8 h = 1; h <= 4; h++) {
                        if (h > 2)
                            break;
                        for (uint8 k = 10; k < 12; k++) {
                            uint8 x = 6;
                            do {
                                x -= 1;
                                if (x == 0)
                                    break;
                                uint8 y = 10;
                                while (y < 17) {
                                    y += 1;
                                }
                            } while (true);
                        }
                    }
                } while (true);
                j *= 2;
            }
        }
        return 1;
    }
}
