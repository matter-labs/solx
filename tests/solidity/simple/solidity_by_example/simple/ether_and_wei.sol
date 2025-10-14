//! { "cases": [ {
//!     "name": "oneWei",
//!     "inputs": [
//!         {
//!             "method": "oneWei",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! }, {
//!     "name": "isOneWei",
//!     "inputs": [
//!         {
//!             "method": "isOneWei",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! }, {
//!     "name": "oneEther",
//!     "inputs": [
//!         {
//!             "method": "oneEther",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1000000000000000000"
//!     ]
//! }, {
//!     "name": "isOneEther",
//!     "inputs": [
//!         {
//!             "method": "isOneEther",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT
pragma solidity >=0.4.16;

contract Test {
    uint public oneWei = 1 wei;
    // 1 wei is equal to 1
    bool public isOneWei = 1 wei == 1;

    uint public oneEther = 1 ether;
    // 1 ether is equal to 10^18 wei
    bool public isOneEther = 1 ether == 1e18;
}
