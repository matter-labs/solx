//! { "cases": [ {
//!     "name": "main",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "9"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    function malloc(uint16 size) private pure returns(uint24) {
        uint24 memory_pointer = 2 * 32;
        uint24 return_pointer;
        assembly {
            return_pointer := mload(memory_pointer)
            mstore(memory_pointer, add(return_pointer, mul(size, 32)))
        }
        return return_pointer;
    }

    function main() public pure returns(uint8) {
        uint24 memory_pointer = 2 * 32;
        assembly {
            mstore(memory_pointer, 0x80)
        }

        return uint8(malloc(1) / 32) + uint8(malloc(1) / 32);
    }
}
