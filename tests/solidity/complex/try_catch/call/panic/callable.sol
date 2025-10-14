// SPDX-License-Identifier: MIT

pragma solidity >=0.7.0;

contract Callable {
    enum A {
        A
    }

    uint[] p;

    bytes b;

    function panic(uint reason) private {
        if (reason == 0x01) {
            assert(false);
        } else if (reason == 0x11) {
            uint a = 0;
            uint b = 5;
            a -= b;
        } else if (reason == 0x12) {
            uint a = 5;
            uint b = 0;
            a %= b;
        } else if (reason ==  0x21) {
            int a = -5;
            A a0 = A(a);
        } else if (reason == 0x22) {
            assembly {
                sstore(b.slot, 3)
            }
            b[0];
        } else if (reason == 0x31) {
            p.pop();
        } else if (reason == 0x32) {
            uint[2] memory a;
            uint b = 5;
            a[b]++;
        } else if (reason == 0x41) {
            uint[] memory a;
            uint b = 2<<100;
            a = new uint[](b);
        }
    }

    function f(uint reason) public {
        panic(reason);
    }
}
