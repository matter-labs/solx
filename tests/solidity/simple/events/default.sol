//! { "cases": [ {
//!     "name": "default",
//!     "inputs": [
//!         {
//!             "method": "test",
//!             "calldata": [
//!                 "42"
//!             ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [
//!         ],
//!         "events": [
//!             {
//!                 "topics": ["0x742ddab21c2fcfa13cae111a0cd043fb6c4375a90b69212466f265cea02c6dbc"],
//!                 "values": []
//!             },
//!             {
//!                 "topics": ["0x0a24cd290a359e539b7a80b016b98ec9c4f4cae46d04d989ed6ae93e8c52c209"],
//!                 "values": ["42", "3735928559"]
//!             },
//!             {
//!                 "topics": [
//!                     "0x37f8d1a8dd2ace10b09f68d60e4c035f242c1273a124f3e2a67bdb87df62b878",
//!                     "0x000000000000000000000000000000000000000000000000000000000000002a"
//!                 ],
//!                 "values": ["12648430"]
//!             }
//!         ]
//!     }
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.21;

contract Test {
    event Trivial();
    event Simple(uint256 Number, address Account);
    event Indexed(uint256 indexed Number, address Account);

    function test(uint256 number) public {
        emit Trivial();
        emit Simple(number, address(0xdeadbeef));
        emit Indexed(number, address(0xc0ffee));
    }
}
