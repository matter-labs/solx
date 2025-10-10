// SPDX-License-Identifier: MIT

pragma solidity >=0.7.0;

import "./storage.sol";

contract Main {
    uint initial;
    Storage _storage;

    constructor(uint _initial, Storage __storage) {
        initial = _initial;
        _storage = __storage;
    }

    function main(uint key, uint value) public returns(uint) {
        _storage.set(key, value);
        return _storage.get(key) + initial;
    }
}
