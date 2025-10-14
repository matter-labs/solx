// SPDX-License-Identifier: MIT

pragma solidity >=0.8.20;

contract Dummy {
    fallback() external {
        assembly {
            return(0, 0)
        }
    }
}
