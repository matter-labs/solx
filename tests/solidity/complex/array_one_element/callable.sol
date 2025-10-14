// SPDX-License-Identifier: MIT

pragma solidity >=0.6.9;

contract Callable {
    function f(uint[1] memory p1) public pure returns(uint) {
        return p1[0];
    }
}
