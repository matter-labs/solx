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
        Str1 memory p1 = Str1(1);
        Str2[2] memory p2 = [
            Str2([Str1(4), Str1(9)], 16),
            Str2([Str1(25), Str1(36)], 49)
        ];
        uint208 p3 = 64;
        int8 p4 = 81;
        bytes1 p5 = 0x64;
        uint256 p6 = 121;
        int32 p7 = 144;
        uint24 p8 = 169;
        uint128 p9 = 196;
        int128 p10 = 225;
        uint72[2] memory p11 = [uint72(256), uint72(289)];
        uint256 p12 = 324;
        int128[1] memory p13 = [int128(361)];
        Str1[1] memory p14 = [Str1(400)];
        bytes5 p15 = 0x00000001B9;
        bytes2 p16 = 0x01E4;
        int256 p17 = 529;
        string[2] memory p18 = ["ab", "cd"];
        bytes memory p19 = "\x01\x02";
        Enum1 p20 = Enum1.B;

        bool result = true;
        for (uint8 i = 1; i <= 23; i++) {
            result = result && main(i, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16, p17, p18, p19, p20) == uint256(i) * uint256(i);
        }

        result = result && main(0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16, p17, p18, p19, p20) == 4522;
        result = result && main(24, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16, p17, p18, p19, p20) == 98;
        result = result && main(25, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16, p17, p18, p19, p20) == 100;
        result = result && main(26, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16, p17, p18, p19, p20) == 2;
        result = result && main(27, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16, p17, p18, p19, p20) == 1;
        result = result && main(28, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16, p17, p18, p19, p20) == 6417941967197257382729677832768998657228800000000;

        return result;
    }

    struct Str1 {
        uint72 a;
    }

    struct Str2 {
        Str1[2] a;
        uint24 b;
    }

    enum Enum1 {
        A,
        B
    }

    function main(
        uint8 param,
        Str1 memory p1,
        Str2[2] memory p2,
        uint208 p3,
        int8 p4,
        bytes1 p5,
        uint256 p6,
        int32 p7,
        uint24 p8,
        uint128 p9,
        int128 p10,
        uint72[2] memory p11,
        uint256 p12,
        int128[1] memory p13,
        Str1[1] memory p14,
        bytes5 p15,
        bytes2 p16,
        int256 p17,
        string[2] memory p18,
        bytes memory p19,
        Enum1 p20
    )
    private
    pure
    returns(uint256 result)
    {
        if (param == 0) {
            result = p1.a +
                p2[0].a[0].a + p2[0].a[1].a + p2[0].b + p2[1].a[0].a + p2[1].a[1].a + p2[1].b +
                p3 +
                uint8(p4) +
                uint8(p5) +
                p6 +
                uint32(p7) +
                p8 +
                p9 +
                uint128(p10) +
                p11[0] + p11[1] +
                p12 +
                uint128(p13[0]) +
                p14[0].a +
                uint40(p15) +
                uint16(p16) +
                uint256(p17) +
                uint8(bytes(p18[0])[0]) + uint8(bytes(p18[1])[0]) +
                uint8(bytes1(p19[0])) +
                uint256(p20);
        } else if (param == 1) {
            result = p1.a;
        } else if (param == 2) {
            result = p2[0].a[0].a;
        } else if (param == 3) {
            result = p2[0].a[1].a;
        } else if (param == 4) {
            result = p2[0].b;
        } else if (param == 5) {
            result = p2[1].a[0].a;
        } else if (param == 6) {
            result = p2[1].a[1].a;
        } else if (param == 7) {
            result = p2[1].b;
        } else if (param == 8) {
            result = p3;
        } else if (param == 9) {
            result = uint8(p4);
        } else if (param == 10) {
            result = uint8(p5);
        } else if (param == 11) {
            result = p6;
        } else if (param == 12) {
            result = uint32(p7);
        } else if (param == 13) {
            result = p8;
        } else if (param == 14) {
            result = p9;
        } else if (param == 15) {
            result = uint128(p10);
        } else if (param == 16) {
            result = p11[0];
        } else if (param == 17) {
            result = p11[1];
        } else if (param == 18) {
            result = p12;
        } else if (param == 19) {
            result = uint128(p13[0]);
        } else if (param == 20) {
            result = p14[0].a;
        } else if (param == 21) {
            result = uint40(p15);
        } else if (param == 22) {
            result = uint16(p16);
        } else if (param == 23) {
            result = uint256(p17);
        } else if (param == 24) {
            result = uint8(bytes(p18[0])[1]);
        } else if (param == 25) {
            result = uint8(bytes(p18[1])[1]);
        } else if (param == 26) {
            result = uint8(bytes1(p19[1]));
        } else if (param == 27) {
            result = uint256(p20);
        } else {
            result = p1.a *
                p2[0].a[0].a * p2[0].a[1].a * p2[0].b * p2[1].a[0].a * p2[1].a[1].a * p2[1].b *
                p3 *
                uint8(p4) *
                uint8(p5) *
                p6 *
                uint32(p7) *
                p8 *
                p9 *
                uint128(p10) *
                p11[0] * p11[1] *
                p12 *
                uint128(p13[0]) *
                p14[0].a *
                uint40(p15) *
                uint16(p16) *
                uint256(p17) *
                uint8(bytes(p18[0])[0]) * uint8(bytes(p18[1])[0]) *
                uint8(bytes1(p19[0])) *
                uint256(p20);
        }
    }
}
