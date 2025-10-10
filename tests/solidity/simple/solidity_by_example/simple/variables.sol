//! { "modes": [
//!     "Y >=0.8.1",
//!     "E", "I"
//! ], "cases": [ {
//!     "name": "text",
//!     "inputs": [
//!         {
//!             "method": "doSomething",
//!             "calldata": [
//!             ]
//!         }, {
//!             "method": "text",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0x20",
//!         "5",
//!         "0x48656c6c6f000000000000000000000000000000000000000000000000000000"
//!     ]
//! }, {
//!     "name": "num",
//!     "inputs": [
//!         {
//!             "method": "doSomething",
//!             "calldata": [
//!             ]
//!         }, {
//!             "method": "num",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "123"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT
pragma solidity >=0.4.16;

contract Test {
    // State variables are stored on the blockchain.
    string public text = "Hello";
    uint public num = 123;

    function doSomething() public {
        // Local variables are not saved to the blockchain.
        uint i = 456;

        // Here are some global variables
        uint timestamp = block.timestamp; // Current block timestamp
        address sender = msg.sender; // address of the caller
    }
}
