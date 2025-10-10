// SPDX-License-Identifier: MIT

pragma solidity >=0.8.4;

error Error1();

contract Callable {
    function throwError(uint reason) private {
        if (reason == 1) {
            assert(false);
        } else if (reason == 2) {
            revert("error");
        } else if (reason == 3) {
            revert Error1();
        }
    }

    constructor(uint reason) {
        throwError(reason);
    }
}
