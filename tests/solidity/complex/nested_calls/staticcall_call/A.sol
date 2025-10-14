// SPDX-License-Identifier: MIT

pragma solidity >=0.5.0;

import "./B.sol";

contract A {
    function main(address b, address c) external {
        (bool success, bytes memory result) = b.staticcall(abi.encodeWithSignature("main(address)", c));
        if (!success) {
            assembly { revert(0, 0) }
        }
    }
}
