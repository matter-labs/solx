// SPDX-License-Identifier: MIT

pragma solidity >=0.7.0;

contract Callable {
    function throwError(uint reason) private {
        if (reason == 1) {
            revert();
        } else if (reason == 2) {
            revert("error1");
        } else if (reason == 3) {
            require(false);
        } else if (reason == 4) {
            require(false, "error2");
        } else if (reason == 5) {
            Callable callable = Callable(0x0000000000000000000000000000000000000003);
            callable.f();
        }
    }

    function f() public pure {}

    constructor(uint reason) {
        throwError(reason);
    }
}
