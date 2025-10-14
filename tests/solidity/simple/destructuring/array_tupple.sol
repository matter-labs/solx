//! { "cases": [ {
//!     "name": "first",
//!     "inputs": [
//!         {
//!             "method": "f1",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1", "1"
//!     ]
//! }, {
//!     "name": "second",
//!     "inputs": [
//!         {
//!             "method": "f2",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1", "2",
//!         "1", "2"
//!     ]
//! }, {
//!     "name": "third",
//!     "inputs": [
//!         {
//!             "method": "f3",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "3", "4",
//!         "1", "2"
//!     ]
//! }, {
//!     "name": "fourth",
//!     "inputs": [
//!         {
//!             "method": "f4",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "3", "4",
//!         "1", "2"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.22;

contract Test {
    uint256[2] public a;
    uint256[2] public b;

    function f1() public returns (uint256[2] memory) {
        a = [uint256(1), 2];
        (a, a[1]) = (a, a[0]);

        return a;
    }

    function f2() public returns (uint256[2] memory, uint256[2] memory) {
        a = [uint256(1), 2];
        b = [uint256(3), 4];
        (a, b) = (b, a);

        return (a, b);
    }

    function f3() public returns (uint256[2] memory, uint256[2] memory) {
        uint256[2] memory a1 = [uint256(1), 2];
        uint256[2] memory b1 = [uint256(3), 4];
        (a1, b1) = (b1, a1);

        return (a1, b1);
    }

    function f4() public returns (uint256[2] memory, uint256[2] memory) {
        a = [uint256(1), 2];
        uint256[2] memory b1 = [uint256(3), 4];
        (a, b1) = (b1, a);

        return (a, b1);
    }
}
