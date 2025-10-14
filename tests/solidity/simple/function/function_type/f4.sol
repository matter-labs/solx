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
//!         "11"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.8.20;

contract Test {
    uint a = 10;
    function() external returns (uint256) ft2;
    TestA testA;

    constructor() {
        testA = new TestA();
    }

    function f() public returns (uint256) {
        ft2 = this.f2;

        try testA.fA(ft2) returns (uint256 r) {
            return r;
        } catch {
            return 100;
        }
    }

    function f2() public returns (uint256) {
        (, bytes memory data) = address(new TestB()).delegatecall(
            abi.encodeWithSignature("fB()")
        );
        return (abi.decode(data, (uint256)));
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
    uint a = 5;
    function() external returns (uint256) ftB1;

    function fB() public returns (uint) {
        return a;
    }
}
