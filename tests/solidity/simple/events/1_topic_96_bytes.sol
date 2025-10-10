//! { "cases": [ {
//!     "name": "default",
//!     "inputs": [
//!         {
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
//!                     "9876543210"
//!                 ],
//!                 "values": [
//!                     "1234567890",
//!                     "1234567891",
//!                     "1234567892"
//!                 ]
//!             }
//!         ]
//!     }
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.21;

contract Test {
    event Event(uint256 indexed, uint256, uint256, uint256) anonymous;

    function test() public {
        emit Event(9876543210, 1234567890, 1234567891, 1234567892);
    }
}
