//! {
//!     "cases": [
//!         {
//!             "name": "main",
//!             "inputs": [
//!                 {
//!                     "method": "main",
//!                     "calldata": []
//!                 }
//!             ],
//!             "expected": {
//!                 "return_data": [],
//!                 "exception": true
//!             }
//!         }
//!     ]
//! }

// SPDX-License-Identifier: MIT

pragma solidity >=0.6.0;

interface ITest2 {
    function f1() external returns (bool);
}

contract Test {
    Test2 ins;

    constructor() public {
        ins = new Test2();
    }

    function main() public returns (bool) {
        try ITest2(address(ins)).f1() returns (bool a) {
            return true;
        } catch {
            return false;
        }
    }
}

contract Test2 {
    function f1() public {
        // return nothing
    }
}
