// SPDX-License-Identifier: MIT

pragma solidity >=0.7.0;

contract Main {
    function f(bool a) public pure {
        require(!a);
    }

    function entry(bool a) public view returns(uint) {
        try this.f(a) {
            return 1;
        } catch {
            return 2;
        }
    }
}