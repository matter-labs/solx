//! { "modes": [ "Y" ], "cases": [ {
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
    function entry() public pure returns(bool) {
        uint8 p1 = 1;
        uint16 p2 = 4;
        uint24 p3 = 9;
        uint32 p4 = 16;
        uint40 p5 = 25;
        uint48 p6 = 36;
        uint56 p7 = 49;
        uint64 p8 = 64;
        uint72 p9 = 81;
        uint80 p10 = 100;
        uint88 p11 = 121;
        uint96 p12 = 144;
        uint104 p13 = 169;
        uint112 p14 = 196;
        uint120 p15 = 225;
        uint128 p16 = 256;
        uint136 p17 = 289;
        uint144 p18 = 324;
        uint152 p19 = 361;
        uint256 p20 = 400;

        bool result = true;

        for (uint8 i = 1; i <= 20; i++) {
            result = result && main(i, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16, p17, p18, p19, p20) == uint256(i) * i;
        }

        result = result && main(0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16, p17, p18, p19, p20) == 2870;
        result = result && main(21, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16, p17, p18, p19, p20) == 5919012181389927685417441689600000000;

        return result;
    }

    function main(
        uint8 param,
        uint8 p1,
        uint16 p2,
        uint24 p3,
        uint32 p4,
        uint40 p5,
        uint48 p6,
        uint56 p7,
        uint64 p8,
        uint72 p9,
        uint80 p10,
        uint88 p11,
        uint96 p12,
        uint104 p13,
        uint112 p14,
        uint120 p15,
        uint128 p16,
        uint136 p17,
        uint144 p18,
        uint152 p19,
        uint256 p20
    )
        private
        pure
        returns(uint256 result)
    {
        if (param == 0) {
            result = p1 + p2 + p3 + p4 + p5 + p6 + p7 + p8 + p9 + p10 + p11 + p12 + p13 + p14 + p15 + p16 + p17 + p18 + p19 + p20;
        } else if (param == 1) {
            result = p1;
        } else if (param == 2) {
            result = p2;
        } else if (param == 3) {
            result = p3;
        } else if (param == 4) {
            result = p4;
        } else if (param == 5) {
            result = p5;
        } else if (param == 6) {
            result = p6;
        } else if (param == 7) {
            result = p7;
        } else if (param == 8) {
            result = p8;
        } else if (param == 9) {
            result = p9;
        } else if (param == 10) {
            result = p10;
        } else if (param == 11) {
            result = p11;
        } else if (param == 12) {
            result = p12;
        } else if (param == 13) {
            result = p13;
        } else if (param == 14) {
            result = p14;
        } else if (param == 15) {
            result = p15;
        } else if (param == 16) {
            result = p16;
        } else if (param == 17) {
            result = p17;
        } else if (param == 18) {
            result = p18;
        } else if (param == 19) {
            result = p19;
        } else if (param == 20) {
            result = p20;
        } else {
            result = p1 * p2 * p3 * p4 * p5 * p6 * p7 * p8 * p9 * p10 * p11 * p12 * p13 * p14 * p15 * p16 * p17 * p18 * p19 * p20;
        }
    }
}
