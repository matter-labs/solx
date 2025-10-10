//! { "cases": [ {
//!     "name": "without_argument",
//!     "inputs": [
//!         {
//!             "method": "main()",
//!             "caller": "0xdeadbeef00000000000000000000000000000042",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0xdeadbeef00000000000000000000000000000042"
//!     ]
//! }, {
//!     "name": "with_argument",
//!     "inputs": [
//!         {
//!             "method": "main(address)",
//!             "calldata": [
//!                 "0xdeadbeef00000000000000000000000000000042"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0xdeadbeef00000000000000000000000000000042"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

// Report https://linear.app/matterlabs/issue/CPR-500/floating-bug-with-a-default-value-for-the-parameter-as-msgsender

pragma solidity >=0.4.16;

contract Test {
    function main(address a) public returns(address) {
        return a;
    }
    function main() external returns(address) {
        return main(msg.sender);
    }
}
