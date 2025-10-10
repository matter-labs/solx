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
    function main() public pure returns(uint8) {
        uint24 memory_pointer = 2 * 32;
        assembly {
            mstore(memory_pointer, 128)
        }

        uint24 pointer_1;
        assembly {
            let return_pointer := mload(memory_pointer)
            mstore(memory_pointer, add(return_pointer, 32))
            pointer_1 := return_pointer
        }

        uint24 pointer_2;
        assembly {
            let return_pointer := mload(memory_pointer)
            mstore(memory_pointer, add(return_pointer, 32))
            pointer_2 := return_pointer
        }

        return uint8(pointer_1 / 32) + uint8(pointer_2 / 32);
    }
}
