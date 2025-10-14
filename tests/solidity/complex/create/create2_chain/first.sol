// SPDX-License-Identifier: MIT

pragma solidity >=0.6.2;

import "./second.sol";

contract First {
    function f(uint256 p) public returns(uint) {
        Second second = new Second{salt: bytes32("second")}();
        return second.f(p);
    }
}
