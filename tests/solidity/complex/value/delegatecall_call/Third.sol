// SPDX-License-Identifier: MIT OR Apache-2.0

pragma solidity ^0.8.0;

contract Third {
    function value() public payable returns(uint256) {
        return msg.value;
    }
}
