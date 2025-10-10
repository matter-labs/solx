//! { "cases": [ {
//!     "name": "default",
//!     "inputs": [
//!         {
//!             "method": "test",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

contract Test {
    function test() external {
        bool s;
        bytes32 addr = bytes32(uint256(uint160(address(this))) + 2**255);
        bytes32 data;
        assembly {
            s := call(gas(), addr, 0, 0, 0, 0 ,32)
            data := mload(0)
        }
        require(s);
        require(data == 0xc569da386df5ba251005c3721bdc1193e6297b5f8fa3a6d32597a6fe93df5397);
    }

    fallback() external {
        assembly {
            mstore(0, 0xc569da386df5ba251005c3721bdc1193e6297b5f8fa3a6d32597a6fe93df5397)
            return(0, 32)
        }
    }
}
