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
//!         "0"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.6.0;

interface ITest2 {
    function f1() external;
}

contract Test {
    Test2 ins;

    constructor() public {
        ins = new Test2();
    }

    function main() public returns (bool) {
        try ITest2(address(ins)).f1() {
            return true;
        } catch {
            return false;
        }
    }
}

contract Test2 {
    event f1();
}
