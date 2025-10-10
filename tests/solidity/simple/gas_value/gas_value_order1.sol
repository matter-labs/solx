//! { "cases": [ {
//!     "name": "zero",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!             ],
//!             "value": "10 wei"
//!         }
//!     ],
//!     "expected": [
//!         "0", "1"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.7.0;

contract Test {
    uint256 public a;
    Test2 ins;

    constructor() {
        ins = new Test2();
    }

    function main() external payable returns (uint256, uint256) {
        return ins.f5{value: a, gas: f1c()}(a);
    }

    function f1c() public returns (uint256) {
        a = 1;
        return 1000000;
    }
}

contract Test2 {
    function f5(uint256 _a) public payable returns (uint256, uint256) {
        return (msg.value, _a);
    }
}