// SPDX-License-Identifier: MIT

pragma solidity >=0.7.0;

contract Storage {
    mapping(uint => uint) s;

    function set(uint key, uint value) public {
        s[key] = value;
    }

    function get(uint key) public view returns(uint) {
        return s[key];
    }
}
