// SPDX-License-Identifier: MIT OR Apache-2.0

pragma solidity ^0.8.0;

contract TestContract {
    uint256 immutable public x;

    constructor(uint256 _x) {
        x = _x;
    }

    function delegate(address _to, bytes memory _data) external returns(uint256) {
        (bool success, bytes memory returnData) = _to.delegatecall(_data);
        require(success);
        assembly {
            return(add(returnData, 32), mload(returnData))
        }
    }
}
