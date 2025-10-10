// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Callable {
    function f(uint a) public pure returns(uint) {
        return a * 2;
    }
}

contract Main {
    function main(Callable callable) public returns(uint) {
        return callable.f(5);
    }
}
