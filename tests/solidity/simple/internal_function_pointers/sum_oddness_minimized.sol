//! { "modes": [ "Y", "E" ], "cases": [ {
//!     "name": "first",
//!     "inputs": [
//!         {
//!             "method": "first",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [
//!             "84"
//!         ]
//!     }
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.6.0;

contract Test {
    function(uint256) internal pure returns (uint256) operation;

    function stupidNoop(uint256 result) internal pure returns(uint256) {
        return result;
    }

    function mainRecursive(uint256 i, uint256 a) private returns(uint256) {
        if (i > a) {
            return 42;
        }
        return mainRecursive(i + 1, a);
    }

    function main(uint256 a) private returns(uint256) {
        // remove this
        operation = stupidNoop;
        return mainRecursive(1, a);
    }

    function first() public returns(uint256) {
        // or one of these
        return main(100) + main(100);
    }
}
