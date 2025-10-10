// SPDX-License-Identifier: MIT

pragma solidity >=0.7.0;

import "./callable.sol";

contract Main {
    function withoutMessage(uint reason) public returns(bool) {
        try new Callable(reason) {
            return true;
        } catch {
            return false;
        }
    }

    function withMessage(uint _reason) public returns(bytes memory) {
        try new Callable(_reason) {
            return "";
        } catch(bytes memory reason) {
            return reason;
        }
    }

    function withStringMessage(uint _reason) public returns(string memory) {
        try new Callable(_reason) {
            return "";
        } catch Error(string memory reason) {
            return reason;
        } catch {
            return "";
        }
    }
}