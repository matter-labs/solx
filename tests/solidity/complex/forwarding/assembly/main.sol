// SPDX-License-Identifier: MIT

pragma solidity >=0.6.0;

import "./callable.sol";

contract Main {
    fallback() external {
        Callable callable = abi.decode(msg.data, (Callable));
        assembly {
            calldatacopy(0, 0, calldatasize())
            let success := call(gas(), callable, 0, 0, calldatasize(), 0, 0)
            returndatacopy(0, 0, returndatasize())
            return(0, sub(returndatasize(), 0x20))
        }
    }
}
