//! { "cases": [ {
//!     "name": "main",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "55"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.24;

contract Test {
    function main(uint8[11] memory v) public pure returns(uint8) {
        (uint8 a, uint8 b, uint8 c, uint8 d) = (v[0], v[1], v[2], v[3]);
        (uint8 e, uint8 f, uint8 g, uint8 h) = (v[4], v[5], v[6], v[7]);
        (uint8 j, uint8 i, uint8 k) = (v[8], v[9], v[10]);

        a += b;
        a += c;
        a += d;
        a += e;
        a += f;
        a += g;
        a += h;
        a += j;
        a += i;
        a += k;

        return a;
    }
}
