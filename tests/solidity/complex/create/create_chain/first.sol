// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

import "./second.sol";

contract First {
    function f(uint256 p) public returns(uint) {
        Second second = new Second();
        return second.f(p);
    }
}
