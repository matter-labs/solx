// SPDX-License-Identifier: MIT OR Apache-2.0

pragma solidity ^0.8.0;

import "./Third.sol";

contract Second {
    function main(address third) external payable returns(uint256) {
        (bool success, bytes memory returnData) = third.delegatecall(abi.encodeWithSelector(Third.value.selector));
        require(success);
        return (abi.decode(returnData, (uint256)));
    }
}
