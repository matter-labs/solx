//! {
//!     "cases": [
//!         {
//!             "name": "main",
//!             "inputs": [
//!                 {
//!                     "method": "main",
//!                     "calldata": [],
//!                     "value": "1 ETH"
//!                 }
//!             ],
//!             "expected": [
//!                 "0"
//!             ]
//!         }
//!     ]
//! }

// SPDX-License-Identifier: MIT

// `value:` is not recognized by solc <= 0.6.1
pragma solidity >=0.6.2;

interface ITest2 {
    function f1() external payable returns (bool);
}

contract Test {
    Test2 ins;

    constructor() public {
        ins = new Test2();
    }

    function main() public payable returns (bool) {
        try ITest2(address(ins)).f1{value: 2 ether}() returns (bool) {
            return true;
        } catch {
            return false;
        }
    }
}

contract Test2 {
    fallback() external payable {
        assembly {
            return(0, 32)
        }
    }
}
