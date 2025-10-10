// SPDX-License-Identifier: MIT

pragma solidity >=0.8.4;

import "./callable.sol";

contract Main {
    function withoutMessage(uint reason, Callable callable) public returns(bool) {
        try callable.f(reason) {
            return true;
        } catch {
            return false;
        }
    }

    function withMessage(uint _reason, Callable callable) public returns(bytes memory) {
        try callable.f(_reason) {
            return "";
        } catch(bytes memory reason) {
            return reason;
        }
    }
}
