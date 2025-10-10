//! { "cases": [ {
//!     "name": "simple",
//!     "inputs": [
//!         {
//!             "method": "simple",
//!             "calldata": [
//!                 "10"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "10"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.24;

contract Test {
    function simple(uint64 value) public pure returns(uint64) {
        (uint24 ptr, uint16 len) = _new(); // if init without separate function it works

        assembly {
            mstore(ptr, value)
        }

        len += 1; // if comment this line it will work
        return get(ptr, len, 0); // if get without function (just *vec.0) it works
    }

    function _new() private pure returns(uint24, uint16) {
        uint24 ptr = 128;
        assembly {
            mstore(64, ptr)
        }
        return (ptr, 0);
    }

    function get(uint24 ptr, uint16 len, uint16 index) private pure returns(uint64 result) {
        assembly {
            result := mload(add(ptr, mul(index, 32)))
        }
    }
}
