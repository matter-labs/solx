//! { "cases": [ {
//!     "name": "testCompileVsRuntime",
//!     "inputs": [
//!         {
//!             "method": "testCompileVsRuntime",
//!             "calldata": [
//!                 "1"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1",
//!         "0"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    function testCompileVsRuntime(uint a) public pure returns (uint, uint) {
        return (
            1/2*2,
            a/2*2
        );
    }
}
