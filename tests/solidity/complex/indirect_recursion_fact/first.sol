// SPDX-License-Identifier: MIT

pragma solidity >=0.4.12;

import "./second.sol";

contract First {
    function fact(uint n, First first, Second second) public returns(uint) {
        if (n == 0) {
            return 1;
        }
        return second.fact(n - 1, first, second) * n;
    }
}
