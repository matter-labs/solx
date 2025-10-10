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
        Str1 memory p1 = Str1(1, [uint256(4), uint256(9)]);

        Str2[2] memory p2 = [
            Str2(
                Str1(16, [uint256(25), uint256(36)]),
                49,
                Enum1.A
            ),
            Str2(
                Str1(81, [uint256(100), uint256(121)]),
                144,
                Enum1.B
            )
        ];

        uint216 p3 = 196;

        Str1[3] memory p4 = [
            Str1(225, [uint256(256), uint256(289)]),
            Str1(324, [uint256(361), uint256(400)]),
            Str1(441, [uint256(484), uint256(529)])
        ];

        Str2 memory p5 = Str2(
            Str1(576, [uint256(625), uint256(676)]),
            729,
            Enum1.C
        );

        uint176 p6 = 841;

        Enum1 p7 = Enum1.D;

        Str1[1] memory p8 = [
            Str1(961, [uint256(1024), uint256(1089)])
        ];

        uint120 p9 = 1156;

        Enum1[3] memory p10 = [
            Enum1.E,
            Enum1.F,
            Enum1.G
        ];

        uint72[2] memory p11 = [uint72(1444), uint72(1521)];

        bool result = true;

        for (uint64 i = 1; i <= 39; i++) {
            result = result && main(i, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11) == uint256(i) * uint256(i);
        }

        result = result && main(0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11) == 0;
        result = result && main(40, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11) == 0;

        if (result) {
            return 1;
        } else {
            return 0;
        }
    }

    struct Str1 {
        uint48 a;
        uint256[2] b;
    }

    enum Enum1 {
        A,
        B,
        C,
        D,
        E,
        F,
        G
    }

    function toUint(Enum1 a) private pure returns(uint result) {
        if (a == Enum1.A)
            result = 64;
        else if (a == Enum1.B)
            result = 169;
        else if (a == Enum1.C)
            result = 784;
        else if (a == Enum1.D)
            result = 900;
        else if (a == Enum1.E)
            result = 1225;
        else if (a == Enum1.F)
            result = 1296;
        else
            result = 1369;
    }

    struct Str2 {
        Str1 a;
        uint88 b;
        Enum1 c;
    }

    function main(
        uint64 param,
        Str1 memory p1,
        Str2[2] memory p2,
        uint216 p3,
        Str1[3] memory p4,
        Str2 memory p5,
        uint176 p6,
        Enum1 p7,
        Str1[1] memory p8,
        uint120 p9,
        Enum1[3] memory p10,
        uint72[2] memory p11
    )
        private
        pure
        returns(uint256 result)
    {
        if (param == 1) {
            result = p1.a;
        } else if (param == 2) {
            result = p1.b[0];
        } else if (param == 3) {
            result = p1.b[1];
        } else if (param == 4) {
            result = p2[0].a.a;
        } else if (param == 5) {
            result = p2[0].a.b[0];
        } else if (param == 6) {
            result = p2[0].a.b[1];
        } else if (param == 7) {
            result = p2[0].b;
        } else if (param == 8) {
            result = toUint(p2[0].c);
        } else if (param == 9) {
            result = p2[1].a.a;
        } else if (param == 10) {
            result = p2[1].a.b[0];
        } else if (param == 11) {
            result = p2[1].a.b[1];
        } else if (param == 12) {
            result = p2[1].b;
        } else if (param == 13) {
            result = toUint(p2[1].c);
        } else if (param == 14) {
            result = p3;
        } else if (param == 15) {
            result = p4[0].a;
        } else if (param == 16) {
            result = p4[0].b[0];
        } else if (param == 17) {
            result = p4[0].b[1];
        } else if (param == 18) {
            result = p4[1].a;
        } else if (param == 19) {
            result = p4[1].b[0];
        } else if (param == 20) {
            result = p4[1].b[1];
        } else if (param == 21) {
            result = p4[2].a;
        } else if (param == 22) {
            result = p4[2].b[0];
        } else if (param == 23) {
            result = p4[2].b[1];
        } else if (param == 24) {
            result = p5.a.a;
        } else if (param == 25) {
            result = p5.a.b[0];
        } else if (param == 26) {
            result = p5.a.b[1];
        } else if (param == 27) {
            result = p5.b;
        } else if (param == 28) {
            result = toUint(p5.c);
        } else if (param == 29) {
            result = p6;
        } else if (param == 30) {
            result = toUint(p7);
        } else if (param == 31) {
            result = p8[0].a;
        } else if (param == 32) {
            result = p8[0].b[0];
        } else if (param == 33) {
            result = p8[0].b[1];
        } else if (param == 34) {
            result = p9;
        } else if (param == 35) {
            result = toUint(p10[0]);
        } else if (param == 36) {
            result = toUint(p10[1]);
        } else if (param == 37) {
            result = toUint(p10[2]);
        } else if (param == 38) {
            result = p11[0];
        } else if (param == 39) {
            result = p11[1];
        } else {
            result = 0;
        }
    }
}
