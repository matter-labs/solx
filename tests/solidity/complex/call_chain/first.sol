// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

import "./second.sol";

contract First {
    function f(uint p, Second second, Third third) public returns(uint) {
        return second.f(p, third);
    }
}
