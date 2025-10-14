// SPDX-License-Identifier: MIT OR Apache-2.0

pragma solidity ^0.8.0;

import "./Second.sol";

contract First {
    function main(address second, address third, uint256 value_next) external payable returns(uint256) {
        (bool success, bytes memory returnData) = second.delegatecall(abi.encodeWithSelector(Second.main.selector, third, value_next));
        require(success);
        return (abi.decode(returnData, (uint256)));
    }
}
