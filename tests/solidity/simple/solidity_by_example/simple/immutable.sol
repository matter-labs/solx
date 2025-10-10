//! { "cases": [ {
//!     "name": "address",
//!     "inputs": [
//!         {
//!             "method": "#deployer",
//!             "calldata": [
//!                 "9823"
//!             ],
//!             "expected": [
//!                 "Test.address"
//!             ]
//!         }, {
//!             "method": "MY_ADDRESS",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0xdeadbeef00000000000000000000000000000001"
//!     ]
//! }, {
//!     "name": "uint",
//!     "inputs": [
//!         {
//!             "method": "#deployer",
//!             "calldata": [
//!                 "555555"
//!             ],
//!             "expected": [
//!                 "Test.address"
//!             ]
//!         }, {
//!             "method": "MY_UINT",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "555555"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.8.0;

contract Test {
    // coding convention to uppercase constant variables
    address public immutable MY_ADDRESS;
    uint public immutable MY_UINT;

    constructor(uint _myUint) {
        MY_ADDRESS = msg.sender;
        MY_UINT = _myUint;
    }
}

