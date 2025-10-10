// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

import "./square.sol";

contract Main {
    function main(uint n, Square square) public returns(uint) {
        uint result = 0;
        for (uint i = 1; i <= n; i++) {
            result += square.square(i);
        }
        return result;
    }
}
