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
//!         "1", "0"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.7.0;

contract Test {
    Test2 public ins;

    constructor() {
        ins = new Test2();
    }

    function main() external payable returns (uint256, uint256) {
        Test2 oldInstance = ins;
        ins.f5{value: 1, gas: f1c()}();
        return (address(oldInstance).balance, address(ins).balance);
    }

    function f1c() public returns (uint256) {
        ins = new Test2();
        return 1000000;
    }
}

contract Test2 {
    function f5() public payable {}
}
