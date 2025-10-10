// SPDX-License-Identifier: MIT

pragma solidity >=0.5.0;

import "./C.sol";

contract B {
    function main(address c) external returns (address) {
        (bool success, bytes memory return_data) = c.delegatecall(abi.encodeWithSignature("main()"));
        address result = abi.decode(return_data, (address));
        return result;
    }
}
