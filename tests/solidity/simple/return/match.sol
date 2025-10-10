//! { "cases": [ {
//!     "name": "entry0",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "0"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "4"
//!     ]
//! }, {
//!     "name": "entry1",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "1"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "5"
//!     ]
//! }, {
//!     "name": "entry2",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "2"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "6"
//!     ]
//! }, {
//!     "name": "entry3",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "3"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "7"
//!     ]
//! }, {
//!     "name": "entry4",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "7"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "99"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    function main(uint8 param) public pure returns(uint64 result) {
        assembly {
            switch div(param, 3)
            case 0 {
                switch div(param, 2)
                case 0 {
                    switch param
                    case 0 {
                        result := 4
                    }
                    case 1 {
                        result := mul(param, 5) // 5
                    }
                    default {
                        result := 42
                    }
                }
                default {
                    result := mul(param, 3) // 6
                }
            }
            default {
                switch param
                case 3 {
                    result := add(mul(param, 2), 1) // 7
                }
                default {
                    result := 99
                }
            }
        }
    }
}