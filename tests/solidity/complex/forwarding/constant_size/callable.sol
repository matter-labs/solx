// SPDX-License-Identifier: MIT

pragma solidity >=0.6.0;

contract Callable {
    fallback() external {
        assembly {
            calldatacopy(0, 0, calldatasize())
            return(0x20, calldatasize())
        }
    }
}
