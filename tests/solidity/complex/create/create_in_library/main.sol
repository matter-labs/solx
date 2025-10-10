// SPDX-License-Identifier: MIT

pragma solidity >=0.6.2;

import "./library.sol";

contract Main {
    // Test Foo.sol by getting it's name.
    function main(uint256 a) public returns (uint256) {
        Callable callable = Callable(L.f());
        return callable.f(a);
    }
}
