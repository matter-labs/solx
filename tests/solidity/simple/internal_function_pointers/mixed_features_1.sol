//! { "modes": [ "Y", "E" ], "cases": [ {
//!     "name": "entry",
//!     "inputs": [
//!         {
//!             "method": "entry",
//!             "calldata": [ "2" ]
//!         }
//!     ],
//!     "expected": [ "16" ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.6.0;

contract Test {
    struct Data {
        uint256 a;
        string b;
        uint256 c;
    }

    function entry(uint256 x) public pure returns (uint256) {
        Data memory data = Data(x, "Test", x * 2);
        return function1(data, x, x);
    }

    function function1(Data memory data, uint256 a, uint256 b) internal pure returns (uint256) {
        if (a == 0) {
            return data.c;
        } else {
            data.a = data.a * 2;
            return function2(data, a - 1, b);
        }
    }

    function function2(Data memory data, uint256 a, uint256 b) internal pure returns (uint256) {
        if (b == 0) {
            return data.a;
        } else {
            data.c = data.c * 2;
            return function1(data, a, b - 1);
        }
    }
}
