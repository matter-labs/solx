//! { "modes": [
//!     "Y >=0.8.1",
//!     "E", "I"
//! ], "cases": [ {
//!     "name": "first",
//!     "inputs": [
//!         {
//!             "method": "f",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [
//!             "0x08C379A000000000000000000000000000000000000000000000000000000000",
//!             "0x0000002000000000000000000000000000000000000000000000000000000000",
//!             "0x0000000374776f00000000000000000000000000000000000000000000000000",
//!             "0x0000000000000000000000000000000000000000000000000000000000000000"
//!         ],
//!         "exception": true
//!     }
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.24;

contract Test {
    string public errorString = "one";

    function f() public {
        require(false, f1());
    }

    function f1() public returns (string memory) {
        errorString = "two";
        this.f2();
        return "three";
    }

    function f2() public {
        revert(errorString);
    }
}
