// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

interface A {
    function f() external view returns (bool);
}

contract Test {
    function f() external {
        uint256 stableFlag;
        for (uint256 i = 0; i < 1; i++) {
            if (A(address(0)).f()) {
                stableFlag = stableFlag | (1 << i);
            }
        }
    }
}
