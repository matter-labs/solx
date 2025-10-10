//! { "cases": [ {
//!     "name": "main",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0x0000000000000000000000000000000000000000000000000000000000000020",
//!         "0x0000000000000000000000000000000000000000000000000000000000000004",
//!         "0xd845fc9200000000000000000000000000000000000000000000000000000000"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

// Report https://linear.app/matterlabs/issue/CPR-353/minimize-the-try-catch-complex-tests

pragma solidity >=0.8.4;

error Error1();

contract Test {
    function main() public returns(bytes memory) {
        try this.f() {
            return "";
        } catch(bytes memory reason) {
            return reason;
        }
    }

    function f() external {
        revert Error1();
    }
}
