//! { "cases": [ {
//!     "name": "f1",
//!     "inputs": [
//!         {
//!             "method": "f1",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "3",
//!         "0",
//!         "2"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.26;

contract Test {
    uint256 public a1;
    uint256 public a2;
    uint256 public a3;

    constructor() public {
        (a1, a2, a3) = (f2(), a1, a2);
    }

    function f1() public returns (uint256,uint256,uint256) {
        return (a1, a2, a3);
    }

    function f2() public returns(uint256){
        a2 = 2;
        return 3;
    }
}
