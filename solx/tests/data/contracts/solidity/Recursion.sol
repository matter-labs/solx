// SPDX-License-Identifier: Unlicensed

pragma solidity >=0.4.16;

contract Recursion {
    uint64 x;

    constructor(uint8 i) public {
        x = fib(i);
    }

    function fib(uint8 n) public pure returns(uint64) {
        if (n <= 1) {
            return n;
        }

        return fib(n - 1) + fib(n - 2);
    }
}