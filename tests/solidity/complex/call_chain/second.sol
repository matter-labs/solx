// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

import "./third.sol";

contract Second {
    function f(uint p, Third third) public returns(uint) {
        return third.f(p) * 2;
    }
}
