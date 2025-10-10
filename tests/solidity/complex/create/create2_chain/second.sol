// SPDX-License-Identifier: MIT

pragma solidity >=0.6.2;

import "./third.sol";

contract Second {
    function f(uint256 p) public returns(uint) {
        Third third = new Third{salt: bytes32("third")}();
        return third.f(p) * 2;
    }
}
