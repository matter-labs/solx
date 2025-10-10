//! { "cases": [ {
//!     "name": "test",
//!     "inputs": [
//!         {
//!             "method": "f",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "200"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.8.20;

contract Test {
    function() external returns (uint256) ft1;
    TestA testA;

    constructor() {
        testA = new TestA();
        ft1 = this.f1;
        address(new TestB()).delegatecall(abi.encodeWithSignature("fB()"));
    }

    function f() public returns (uint256) {
        try testA.fA(this.f1) returns (uint256 r) {
            return r;
        } catch {
            return 200;
        }
    }

    function f1() public returns (uint256) {
        revert();
    }
}

contract TestA {
    uint256 x = 1;

    function fA(
        function() external returns (uint256) _f
    ) public returns (uint256) {
        return x + _f();
    }
}

contract TestB {
    function() external returns (uint256) ftB1;

    function fB() public {
        ftB1 = this.fB2;
    }

    function fB2() public returns (uint256) {
        return 6;
    }
}
