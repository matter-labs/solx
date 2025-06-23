// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

contract StackTooDeep {
    function boom(
        uint256 a0, uint256 a1, uint256 a2, uint256 a3, uint256 a4,
        uint256 a5, uint256 a6, uint256 a7, uint256 a8, uint256 a9,
        uint256 a10, uint256 a11, uint256 a12, uint256 a13, uint256 a14,
        uint256 a15, uint256 a16
    ) public pure returns (uint256) {
        uint256 x0 = a0 + 1;
        uint256 x1 = a1 + 1;
        uint256 x2 = a2 + 1;
        uint256 x3 = a3 + 1;
        uint256 x4 = a4 + 1;
        uint256 x5 = a5 + 1;
        uint256 x6 = a6 + 1;
        uint256 x7 = a7 + 1;
        uint256 x8 = a8 + 1;
        uint256 x9 = a9 + 1;
        uint256 x10 = a10 + 1;
        uint256 x11 = a11 + 1;
        uint256 x12 = a12 + 1;
        uint256 x13 = a13 + 1;
        uint256 x14 = a14 + 1;
        uint256 x15 = a15 + 1;
        uint256 x16 = a16 + 1;
        return x0 + x1 + x2 + x3 + x4 + x5 + x6 + x7 + x8 + x9 + x10 + x11 + x12 + x13 + x14 + x15 + x16;
    }
}
