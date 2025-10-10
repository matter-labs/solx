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
//!         "1"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    function entry() public pure returns(uint64) {
        bool result = true;
        for (uint8 i = 0; i < 7; i++) {
            result = result && main(i) == i + 4;
        }
        result = result && main(7) == 99;
        result = result && main(11) == 99;
        if (result) {
            return 1;
        } else {
            return 0;
        }
    }

    function main(uint8 param) private pure returns(uint64) {
        for (uint8 i = 0; i < 10; i++) {
            if (param == 0 && i == 4) {
                return i; // 4
            }
            if (param / 3 == 0) {
                uint8 j = 10;
                while (j > 0) {
                    if (param == 1 && j == 5) {
                        return j; // 5
                    }
                    do {
                        if (param == 2) {
                            return 6;
                        } else {
                            break;
                        }
                    } while(true);
                    j -= 1;
                }
            } else {
                if (param < 5) {
                    for (uint8 l = 0; l < 10; l++) {
                        if (param == 3 && l == 7) {
                            return l; // 7
                        }
                    }
                    if (param == 4) {
                        return 8;
                    }
                } else {
                    if (param < 7) {
                        do {
                            if (param == 5) {
                                return 9;
                            } else {
                                return 10;
                            }
                        } while(true);
                    }
                }
            }
        }
        return 99;
    }
}