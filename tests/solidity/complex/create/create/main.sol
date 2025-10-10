// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

import "./callable.sol";

contract Main {
    function main() external returns(uint256) {
        Callable callable = new Callable();

        callable.set(10);
        return callable.get();
    }
}
