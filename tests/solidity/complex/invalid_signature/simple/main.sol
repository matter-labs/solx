// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

import "./icallable.sol";

contract Main {
    function main(address callable) public returns(uint) {
        return ICallable(callable).f(5, false);
    }
}
