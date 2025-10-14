//! { "modes": [
//!     "Y >=0.8.2",
//!     "E", "I"
//! ], "cases": [ {
//!     "name": "empty",
//!     "inputs": [
//!         {
//!             "method": "getSelector",
//!             "calldata": [
//!                 "0x20",
//!                 "7",
//!                 "0x656d707479282900000000000000000000000000000000000000000000000000"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0xf2a75fe400000000000000000000000000000000000000000000000000000000"
//!     ]
//! }, {
//!     "name": "withParams",
//!     "inputs": [
//!         {
//!             "method": "getSelector",
//!             "calldata": [
//!                 "0x20",
//!                 "53",
//!                 "0x77697468506172616d73282875696e743235365b325d2c62797465735b5d292c",
//!                 "0x737472696e672c75696e7431362c627974657332290000000000000000000000"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0x6d3c5ac200000000000000000000000000000000000000000000000000000000"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT
pragma solidity >=0.5.0;

contract Test {
    /*
    "transfer(address,uint256)"
    0xa9059cbb
    "transferFrom(address,address,uint256)"
    0x23b872dd
    */
    function getSelector(string calldata _func) external pure returns (bytes4) {
        return bytes4(keccak256(bytes(_func)));
    }
}
