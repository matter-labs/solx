// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

import "./callable.sol";

contract Main {
    function main(Callable callable) public returns(uint) {
        return callable.f(5);
    }
}
