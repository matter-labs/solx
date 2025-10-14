//! {
//!     "cases": [
//!         {
//!             "name": "first",
//!             "inputs": [
//!                 {
//!                     "method": "#deployer",
//!                     "calldata": []
//!                 }
//!             ],
//!             "expected": {
//!                 "return_data": [
//!                     "Test.address"
//!                 ],
//!                 "events": [
//!                     {
//!                         "topics": [
//!                             "0xb2c71c5e3e8a01f913137c9970f83f0d54f6dc867e164b5de1fcad3ea64fb84f"
//!                         ],
//!                         "values": [
//!                             "1",
//!                             "2"
//!                         ]
//!                     }
//!                 ]
//!             }
//!         }
//!     ]
//! }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.22;

contract Test {
    event E1(uint256 Number1, uint256 Number2);

    constructor() public {
        emit E1(f1(), 2);
    }

    function f1() public returns (uint256) {
        return 1;
    }
}
