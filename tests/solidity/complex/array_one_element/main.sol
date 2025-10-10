// SPDX-License-Identifier: MIT

// Report https://linear.app/matterlabs/issue/CPR-269/call-with-calldata-variable-bug

pragma solidity >=0.6.9;

import "./callable.sol";

contract Main {
    function main(uint[1] calldata p1, Callable callable) public returns(uint) {
        return callable.f(p1);
    }
}
