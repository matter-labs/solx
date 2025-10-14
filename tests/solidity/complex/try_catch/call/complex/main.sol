// SPDX-License-Identifier: MIT

pragma solidity >=0.8.4;

import "./callable.sol";

contract Main {
    function main(uint reason, Callable callable) public returns(bytes memory) {
        try callable.f(reason) {
            return "";
        } catch Error(string memory reason) {
            return bytes(reason);
        } catch(bytes memory reason) {
            return reason;
        }
    }
}
