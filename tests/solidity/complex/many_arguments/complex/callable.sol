// SPDX-License-Identifier: MIT

pragma solidity >=0.6.9;
pragma experimental ABIEncoderV2;

contract Callable {
    enum E1 {
        A, B, C
    }

    struct Str1 {
        uint[2] a;
        uint8 b;
    }

    struct Str2 {
        Str1 a;
        E1 b;
    }

    function f(Str1[2] memory p1, Str2 memory p2, uint24 p3, E1[3] memory p4, uint136[3] memory p5) public pure returns(uint) {
        uint result = 0;
        for (uint i = 0; i < p1.length; i++) {
            for(uint j = 0; j < p1[i].a.length; j++) {
                result += p1[i].a[j];
            }
            result += p1[i].b;
        }

        for (uint j = 0; j < p2.a.a.length; j++) {
            result += p2.a.a[j];
        }
        result += p2.a.b;
        result += uint(p2.b);

        result += p3;

        for (uint i = 0; i < p4.length; i++) {
            result += uint(p4[i]);
        }

        for(uint i = 0; i < p5.length; i++) {
            result += p5[i];
        }

        return result;
    }
}
