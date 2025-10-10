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
//!         "100"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.8.20;

contract Test {
    function() external returns (uint256) ft1;
    function() external returns (uint256) ft2;
    TestA testA;

    constructor() {
        testA = new TestA();
    }

    function f() public returns (uint256) {
        ft1 = this.f1;
        ft2 = this.f2;

        (ft1, ft2) = (ft2, ft1);

        try testA.fA(ft2) returns (uint256 r) {
            return r;
        } catch {
            return 100;
        }
    }

    function f1() public returns (uint256) {
        revert();
    }

    function f2() public returns (uint256) {
        return 2;
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
