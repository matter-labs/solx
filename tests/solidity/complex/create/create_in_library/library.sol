// SPDX-License-Identifier: MIT

pragma solidity >=0.6.2;

import "./callable.sol";

library L {
    function f() public returns(address) {
        return address(new Callable());
    }
}
