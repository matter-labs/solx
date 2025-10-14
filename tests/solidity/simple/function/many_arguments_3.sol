//! { "cases": [ {
//!     "name": "entry",
//!     "inputs": [
//!         {
//!             "method": "entry",
//!             "calldata": [
//!                 "0", "0", "0", "0", "0", "2"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

// Report https://linear.app/matterlabs/issue/CPR-179/many-arguments-bug

pragma solidity >=0.4.16;

contract Test {
    function entry(
        uint256 p1,
        uint256 p2,
        uint256 p3,
        uint256 p4,
        uint256 p5,
        uint256 p6
    ) public pure returns(uint256) {
        if (main(0, p1, p2, p3, p4, p5, p6) == 0 &&
            main(7, p1, p2, p3, p4, p5, p6) == 0)
            return 1;
        else
            return 0;
    }

    function main(
        uint8 param,
        uint256 p1,
        uint256 p2,
        uint256 p3,
        uint256 p4,
        uint256 p5,
        uint256 p6
    )
        private
        pure
        returns(uint256)
    {
        if (param == 1)
            return p1;
        else if (param == 2)
            return p2;
        else if (param == 3)
            return p3;
        else if (param == 4)
            return p4;
        else if (param == 5)
            return p5;
        else if (param == 6)
            return p6;
        else
            return 0;
    }
}
