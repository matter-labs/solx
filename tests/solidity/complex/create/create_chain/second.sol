// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

import "./third.sol";

contract Second {
    function f(uint256 p) public returns(uint) {
        Third third = new Third();
        return third.f(p) * 2;
    }
}
