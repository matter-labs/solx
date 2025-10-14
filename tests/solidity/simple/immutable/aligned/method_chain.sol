//! { "cases": [ {
//!     "name": "main",
//!     "inputs": [
//!         {
//!             "method": "#deployer",
//!             "calldata": [
//!                 "0x05",
//!                 "0x0b"
//!             ],
//!             "expected": [
//!                 "Test.address"
//!             ]
//!         },
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "42"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1024"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.8.0;

contract Test {
    uint256 immutable a;
    uint256 immutable b;

    constructor(uint256 x, uint256 y) {
        a = x;
        b = y;
    }

    function double(uint256 _value) internal pure returns(uint256) {
        return _value * 2;
    }

    function triple(uint256 _value) internal pure returns(uint256) {
        return 3 * _value;
    }

    function quadruple(uint256 _value) internal pure returns(uint256) {
        return _value * 4;
    }

    function main(uint256 _value) external view returns(uint256) {
        return a + quadruple(triple(double(_value))) + b;
    }
}
