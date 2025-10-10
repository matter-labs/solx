//! { "cases": [ {
//!     "name": "f",
//!     "inputs": [
//!         {
//!             "method": "f",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.12;

contract Test {
    address public a1 = a2;
    address public a2 = address(0xff);

    function f() public returns (uint256) {
        assert(a1 == address(0));
        return 0;
    }
}
