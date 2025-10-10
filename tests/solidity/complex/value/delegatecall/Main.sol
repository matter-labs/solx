// SPDX-License-Identifier: MIT OR Apache-2.0

pragma solidity ^0.8.0;

import "./TestContract.sol";

contract Main {
    function main(address testContract) external payable returns(uint256) {
        (bool success, bytes memory returnData) = testContract.delegatecall(abi.encodeWithSelector(TestContract.value.selector));
        require(success);
        return abi.decode(returnData, (uint256));
    }
}
