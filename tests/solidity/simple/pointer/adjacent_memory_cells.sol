//! { "cases": [ {
//!     "name": "_default",
//!     "inputs": [
//!         {
//!             "method": "_default",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "42"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    function _default() public pure returns(uint8 result) {
        uint16 ptr = 42 * 32;
        assembly {
            mstore(add(ptr, 0x20), 0)
            mstore(add(ptr, 0x40), 42)
            mstore(add(ptr, 0x60), 0)

            result := add(mload(add(ptr, 0x20)), add(mload(add(ptr, 0x40)), mload(add(ptr, 0x60))))
        }
    }
}