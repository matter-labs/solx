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
//!                             "0x058c8f482d91fc45c7658465891699998b90846015007b06504ee51389461e82"
//!                         ],
//!                         "values": [
//!                             "3",
//!                             "4"
//!                         ]
//!                     },
//!                     {
//!                         "topics": [
//!                             "1"
//!                         ],
//!                         "values": [
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
    event E1(uint256 indexed Number1, uint256 Number2) anonymous;
    event E2(uint256 Number1, uint256 Number2);

    constructor() public {
        emit E1(f1(), 2);
    }

    function f1() public returns (uint256) {
        emit E2(3, 4);
        return 1;
    }
}
