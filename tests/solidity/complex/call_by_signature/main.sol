// SPDX-License-Identifier: MIT

pragma solidity >=0.4.22;

import "./storage.sol";

contract Main {
    function main(uint value, address storage_address) public returns(uint) {
        storage_address.call(abi.encodeWithSignature("set(uint256)", value));
        Storage _storage = Storage(storage_address);
        return _storage.get() * 2;
    }
}
