// SPDX-License-Identifier: Unlicensed

pragma solidity >=0.4.12;

import "./Callable.sol";

contract Main {
    function main() external returns(uint256) {
        Callable callable = new Callable();

        callable.set(10);
        return callable.get();
    }
}