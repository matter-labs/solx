// SPDX-License-Identifier: MIT

pragma solidity >=0.5.0;

import "./C.sol";

contract B {
    function main(C c) external returns (address) {
        return c.main();
    }
}
