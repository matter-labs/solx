// SPDX-License-Identifier: MIT

pragma solidity >=0.5.0;

import "./B.sol";

contract A {
    function main(address b, address c) external returns (address) {
        (bool success, bytes memory return_data) = b.delegatecall(abi.encodeWithSignature("main(address)", c));
        address result = abi.decode(return_data, (address));
        return result;
    }
}
