// SPDX-License-Identifier: MIT

pragma solidity >=0.5.0;

contract Callable {
    uint256[2] first;
    uint8 second;

    function set(uint256[2] calldata _first, uint8 _second) external {
        first = _first;
        second = _second;
    }

    function get() external view returns(uint256[2] memory, uint8) {
        return (first, second);
    }
}