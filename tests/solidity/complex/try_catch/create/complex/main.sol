// SPDX-License-Identifier: MIT

pragma solidity >=0.8.4;

import "./callable.sol";

contract Main {
    function main(uint reason) public returns(bytes memory) {
        try new Callable(reason) {
            return "";
        } catch Error(string memory reason) {
            return bytes(reason);
        } catch(bytes memory reason) {
            return reason;
        }
    }
}
