//! { "cases": [ {
//!     "name": "default",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "0x99729a298c414e9c2ea7a37539d5527ff363ccb618b432edde0da847ed417f11"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0xdd71feeedc19e712403093645466eb61bdb4502e4d029882f1b3a490c0b838eb"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.8.0;

contract Test {
    uint256 constant a = 0xFFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFC;
    uint256 constant b = 0x5AC635D8AA3A93E7B3EBBD55769886BC651D06B0CC53B0F63BCE3C3E27D2604B;
    uint256 constant p = 0xFFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFF;

    function main(uint256 x) public pure returns (uint) {
        unchecked {
            uint256 RHS = mulmod(mulmod(x, x, p), x, p); // x^3
            RHS = addmod(RHS, mulmod(x, a, p), p); // x^3 + a*x
            RHS = addmod(RHS, b, p); // x^3 + a*x + b

            return RHS;
        }
    }
}
