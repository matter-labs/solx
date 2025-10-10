//! { "cases": [ {
//!     "name": "test",
//!     "inputs": [
//!         {
//!             "caller": "0xDEADBEEF00000000000000000000000000000001",
//!             "method": "test",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [
//!         ],
//!         "events": [
//!             {
//!                 "topics": [
//!                     "0x0738f4da267a110d810e6e89fc59e46be6de0c37b1d5cd559b267dc3688e74e0",
//!                     "0xDEADBEEF00000000000000000000000000000001"
//!                 ],
//!                 "values": [
//!                     "0x20",
//!                     "12",
//!                     "0x48656c6c6f20576f726c64210000000000000000000000000000000000000000"
//!                 ]
//!             }, {
//!                 "topics": [
//!                     "0x0738f4da267a110d810e6e89fc59e46be6de0c37b1d5cd559b267dc3688e74e0",
//!                     "0xDEADBEEF00000000000000000000000000000001"
//!                 ],
//!                 "values": [
//!                     "0x20",
//!                     "10",
//!                     "0x48656c6c6f2045564d2100000000000000000000000000000000000000000000"
//!                 ]
//!             }, {
//!                 "topics": [
//!                     "0xfe1a3ad11e425db4b8e6af35d11c50118826a496df73006fc724cb27f2b99946"
//!                 ],
//!                 "values": []
//!             }
//!         ]
//!     }
//! } ] }

// SPDX-License-Identifier: MIT
pragma solidity >=0.4.21;

contract Test {
    // Event declaration
    // Up to 3 parameters can be indexed.
    // Indexed parameters helps you filter the logs by the indexed parameter
    event Log(address indexed sender, string message);
    event AnotherLog();

    function test() public {
        emit Log(msg.sender, "Hello World!");
        emit Log(msg.sender, "Hello EVM!");
        emit AnotherLog();
    }
}
