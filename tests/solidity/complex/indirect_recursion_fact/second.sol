// SPDX-License-Identifier: MIT

pragma solidity >=0.4.12;

import "./first.sol";

contract Second {
    function fact(uint n, First first, Second second) public returns(uint) {
        if (n == 0) {
            return 1;
        }
        return first.fact(n - 1, first, second) * n;
    }
}
