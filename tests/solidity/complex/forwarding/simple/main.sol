// SPDX-License-Identifier: MIT

pragma solidity >=0.6.0;

import "./callable.sol";

contract Main {
    fallback() external {
        Callable callable = abi.decode(msg.data, (Callable));
        (bool success, bytes memory data) = address(callable).call(msg.data);
        assembly {
            return(add(data, 0x20), sub(returndatasize(), 0x20))
        }
    }
}
