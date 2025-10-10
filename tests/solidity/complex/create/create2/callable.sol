// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Callable {
    uint256 value;

    function set(uint256 x) external {
        value = x;
    }

    function get() external view returns(uint256) {
        return value;
    }
}