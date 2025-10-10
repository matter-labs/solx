//! { "modes": [ "Y", "E" ], "cases": [ {
//!     "name": "entry",
//!     "inputs": [
//!         {
//!             "method": "entry",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    function entry() public pure returns(uint64) {
        uint8 p1 = 1;
        uint16 p2 = 4;
        uint32 p3 = 9;
        uint64 p4 = 16;
        uint72 p5 = 25;
        uint80 p6 = 36;
        uint8 i = 1;

        if (test(i, p1, p2, p3, p4, p5, p6)) {
            return 1;
        } else {
            return 0;
        }
    }

    function test(
        uint8 param,
        uint8 p1,
        uint16 p2,
        uint32 p3,
        uint64 p4,
        uint72 p5,
        uint80 p6
    )
        private
        pure
        returns(bool)
    {
        bool result = true;
        function (uint8, uint8, uint16, uint32, uint64, uint72, uint80) pure returns(uint256) funcPtr = main;
        result = result && funcPtr(param, p1, p2, p3, p4, p5, p6) == param * param;
        
        if(param < 6){
            result = result && test(param + 1, p1, p2, p3, p4, p5, p6);
        }

        if(param == 6) {
            result = result && funcPtr(0, p1, p2, p3, p4, p5, p6) == 91;
            result = result && funcPtr(16, p1, p2, p3, p4, p5, p6) == 91;
        }

        return result;
    }

    function main(
        uint8 param,
        uint8 p1,
        uint16 p2,
        uint32 p3,
        uint64 p4,
        uint72 p5,
        uint80 p6
    )
        private
        pure
        returns(uint256 result)
    {
        if (param == 1) {
            result = p1;
        } else if (param == 2) {
            result = p2;
        } else if (param == 3) {
            result = p3;
        } else if (param == 4) {
            result = p4;
        } else if (param == 5) {
            result = p5;
        } else if (param == 6) {
            result = p6;
        } else {
            result = p1 + p2 + p3 + p4 + p5 + p6;
        }
    }
}
