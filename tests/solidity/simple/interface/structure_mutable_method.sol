//! { "cases": [ {
//!     "name": "main",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "1", "2", "3"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "131"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.19;
pragma experimental ABIEncoderV2;

contract Test {
    struct Data {
        uint8 a;
        uint8 b;
        uint8 c;
    }

    function fill(Data memory data, uint8[3] memory values) private pure returns(Data memory) {
        data.a = values[0];
        data.b = values[1];
        data.c = values[2];
        return data;
    }

    function main(Data memory witness) public pure returns(uint8) {
        Data memory filled = fill(witness, [25, 42, 64]);
        return filled.a + filled.b + filled.c;
    }
}
