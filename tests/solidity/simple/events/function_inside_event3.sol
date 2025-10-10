//! { "cases": [ {
//!     "name": "f",
//!     "inputs": [
//!         {
//!             "method": "f",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [
//!         ],
//!         "events": [
//!             {
//!                 "topics": ["0xb2c71c5e3e8a01f913137c9970f83f0d54f6dc867e164b5de1fcad3ea64fb84f"],
//!                 "values": ["3", "4"]
//!             }, {
//!                 "topics": ["0xb2c71c5e3e8a01f913137c9970f83f0d54f6dc867e164b5de1fcad3ea64fb84f"],
//!                 "values": ["0", "1"]
//!             }
//!         ]
//!     }
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.21;

contract Test {
    event E1(uint256 Number1, uint256 Number2);
    uint public a;

    function f() public {
        emit E1(a, f1());
    }

    function f1() internal returns (uint256) {
        emit E1(3, 4);
        a = 5;
        return 1;
    }
}
