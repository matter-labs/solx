// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

interface ICallable {
    function f(uint a, bool x) external pure returns(uint);
}
