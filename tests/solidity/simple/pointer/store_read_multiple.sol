//! { "cases": [ {
//!     "name": "one",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "5", "42",
//!                 "10", "21",
//!                 "15", "63"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "126"
//!     ]
//! }, {
//!     "name": "two",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "101", "2",
//!                 "102", "4",
//!                 "103", "6"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "12"
//!     ]
//! }, {
//!     "name": "three",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "43", "1",
//!                 "42", "1",
//!                 "41", "1"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "3"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    function main(
        uint16 address1,
        uint8 value1,
        uint16 address2,
        uint8 value2,
        uint16 address3,
        uint8 value3
    )
        public
        pure
        returns(uint8 result)
    {
        uint16 pointer1 = address1 * 32;
        assembly {
            mstore(pointer1, value1)
        }
        
        uint16 pointer2 = address2 * 32;
        assembly {
            mstore(pointer2, value2)
        }
        
        uint16 pointer3 = address3 * 32;
        assembly {
            mstore(pointer3, value3)
        }

        assembly {
            result := add(mload(pointer1), add(mload(pointer2), mload(pointer3)))
        }
    }
}