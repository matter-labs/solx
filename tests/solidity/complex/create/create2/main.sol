// SPDX-License-Identifier: MIT

pragma solidity >=0.6.2;

import "./callable.sol";

contract Main {
    function main(bytes32 salt) external returns(uint256) {
        Callable callable = new Callable{salt: salt}();

        callable.set(10);
        return callable.get();
    }
}
